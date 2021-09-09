use {
    super::{
        context::Context, Pid, Priority, Process, ReceiveFrom, State, LEAST_PRIORITY_LEVEL,
        MAX_PROCESS,
    },
    crate::{interrupt, tss},
    core::convert::TryInto,
    heapless::{Deque, Vec},
    ipc_api::Message,
    spinning_top::{const_spinlock, Spinlock, SpinlockGuard},
    vm::accessor::single::{read_write, ReadWrite},
    x86_64::VirtAddr,
};

static MANAGER: Spinlock<Manager<MAX_PROCESS>> = const_spinlock(Manager::new());

pub(crate) fn switch() {
    // It is impossible to inline `manager`. If it is inlined, the lock will not be unlocked until
    // the end of the `if` statement. When the next process tries to switch to another, it fails to
    // lock `MANAGER` because it is already locked.
    let mut manager = lock();

    if let Some((current_context, next_context)) = manager.try_switch() {
        // The lock is never unlocked after the context switch unless `manager` is dropped here,
        // causing a deadlock on the following process switch.
        drop(manager);

        Context::switch(current_context, next_context);
    }
}

pub(crate) fn send(to: Pid, message: Message) {
    // The kernel-privileged processes call this function directly, so at this point, the
    // interrupts may not be disabled. If a process switch occurs during the execution of this
    // function because of the timer interrupt, the kernel-privileged process still locks the
    // process manager. When the following process tries to switch, it fails to lock the process
    // manager because the preceding kernel-privileged process has already locked it. That is why
    // we disable interrupts while sending a message to avoid a process switch during the execution
    // of this function.
    interrupt::disable_interrupts_and_do(|| {
        send_without_disabling_interrupts(to, message);
    });
}

pub(crate) fn receive(from: ReceiveFrom, buffer: *mut Message) {
    // The kernel-privileged processes call this function directly, so at this point, the
    // interrupts may not be disabled. If a process switch occurs during the execution of this
    // function because of the timer interrupt, the kernel-privileged process still locks the
    // process manager. When the following process tries to switch, it fails to lock the process
    // manager because the preceding kernel-privileged process has already locked it. That is why
    // we disable interrupts while receiving a message to avoid a process switch during the
    // execution of this function.
    interrupt::disable_interrupts_and_do(|| {
        receive_without_disabling_interrupts(from, buffer);
    });
}

pub(super) fn init() {
    lock().init();
}

pub(super) fn process_exists(pid: Pid) -> bool {
    lock().exists(pid)
}

pub(super) fn add(p: Process) {
    lock().add(p);
}

pub(super) fn add_idle() {
    lock().add_idle();
}

#[no_mangle]
fn current_kernel_stack_bottom() -> u64 {
    lock().current_kernel_stack_bottom().as_u64()
}

fn send_without_disabling_interrupts(to: Pid, mut message: Message) {
    message.header.sender_pid = lock().running.as_usize().try_into().unwrap();

    lock().send(to, message);

    // This switch is necessary because the sender may wait for the receiver.
    switch();
}

fn receive_without_disabling_interrupts(from: ReceiveFrom, buffer: *mut Message) {
    // SAFETY: The pointer is not dereferenced.
    lock().receive(from, unsafe { ptr_to_accessor(buffer) });

    // This switch is necessary because the receiver may wait for the sender.
    switch();
}

fn lock<'a>() -> SpinlockGuard<'a, Manager<MAX_PROCESS>> {
    let m = MANAGER.try_lock();

    m.expect("Failed to lock the process manager.")
}

/// # Safety
///
/// The caller must not dereference `p` while the returned accessor is alive.
unsafe fn ptr_to_accessor<T>(p: *mut T) -> ReadWrite<T> {
    let p = VirtAddr::from_ptr(p);
    let p = vm::translate(p);
    let p = p.expect("The address is not mapped.");
    unsafe { read_write(p) }
}

struct Manager<const N: usize> {
    processes: [Option<Process>; N],

    // The running PID is not contained.
    runnable_pids: RunnablePids<{ LEAST_PRIORITY_LEVEL + 1 }>,

    running: Pid,
}
impl<const N: usize> Manager<N> {
    const fn new() -> Self {
        const UNUSED_PROCESS_ENTRY: Option<Process> = None;

        Self {
            processes: [UNUSED_PROCESS_ENTRY; N],
            runnable_pids: RunnablePids::new(),
            running: Pid::new(0),
        }
    }

    fn init(&mut self) {
        self.runnable_pids.init();
    }

    fn add_idle(&mut self) {
        let idle = super::Process::idle();

        assert_eq!(idle.pid, Pid::new(0), "Wrong PID for the idle process.");

        tss::set_kernel_stack_addr(idle.kernel_stack_bottom_addr());

        self.add_to_process_collection(idle);
    }

    fn add(&mut self, p: Process) {
        self.add_to_runnable_pid_queue(&p);
        self.add_to_process_collection(p);
    }

    fn add_to_runnable_pid_queue(&mut self, process: &Process) {
        self.runnable_pids.push(process.pid, process.priority);
    }

    fn add_to_process_collection(&mut self, p: Process) {
        let pid = p.pid;

        let entry = &mut self.processes[pid.as_usize()];

        if entry.is_some() {
            panic!("{} is double-used.", pid);
        } else {
            *entry = Some(p);
        }
    }

    fn exists(&self, pid: Pid) -> bool {
        self.processes[pid.as_usize()].is_some()
    }

    // Do not switch the context inside this method. Otherwise, the lock of `MANAGER` will never be
    // unlocked during the execution of the next process, causing a deadlock.
    fn try_switch(&mut self) -> Option<(*mut Context, *mut Context)> {
        Switcher::from(self).try_switch()
    }

    fn send(&mut self, to: Pid, message: Message) {
        Sender::new(self, to, message).send();
    }

    fn receive(&mut self, from: ReceiveFrom, buffer: ReadWrite<Message>) {
        Receiver::new(self, from, buffer).receive();
    }

    fn wake(&mut self, pid: Pid) {
        let proc = self.process_as_mut(pid);

        assert!(
            !matches!(proc.state, State::Running | State::Runnable),
            "The process is already awake."
        );

        proc.state = State::Runnable;

        let pid = proc.pid;
        let priority = proc.priority;

        self.runnable_pids.push(pid, priority);
    }

    fn current_kernel_stack_bottom(&self) -> VirtAddr {
        self.process_as_ref(self.running).kernel_stack_bottom_addr()
    }

    fn process_as_ref(&self, pid: Pid) -> &Process {
        let proc = self.processes[pid.as_usize()].as_ref();

        proc.unwrap_or_else(|| panic!("No entry for the process with {}", pid))
    }

    fn process_as_mut(&mut self, pid: Pid) -> &mut Process {
        let proc = self.processes[pid.as_usize()].as_mut();

        proc.unwrap_or_else(|| panic!("No entry for the process with {}", pid))
    }
}

struct Switcher<'a, const N: usize>(&'a mut Manager<N>);
impl<const N: usize> Switcher<'_, N> {
    fn try_switch(mut self) -> Option<(*mut Context, *mut Context)> {
        let next = self.update_runnable_pids_and_return_next_pid();

        (self.0.running != next).then(|| self.switch_to(next))
    }

    fn update_runnable_pids_and_return_next_pid(&mut self) -> Pid {
        if self.0.process_as_ref(self.0.running).state == State::Running {
            self.push_current_process_as_runnable();
        }

        self.0.runnable_pids.pop()
    }

    fn push_current_process_as_runnable(&mut self) {
        let process = self.0.process_as_ref(self.0.running);

        let pid = process.pid;
        let priority = process.priority;

        self.0.runnable_pids.push(pid, priority);
    }

    fn switch_to(&mut self, next: Pid) -> (*mut Context, *mut Context) {
        self.check_kernel_stack_guard(self.0.running);
        self.check_kernel_stack_guard(next);

        self.switch_kernel_stack(next);

        if self.0.process_as_ref(self.0.running).state == State::Running {
            self.0.process_as_mut(self.0.running).state = State::Runnable;
        }

        let current = self.0.running;

        self.0.running = next;
        self.0.process_as_mut(next).state = State::Running;

        (self.context(current), self.context(next))
    }

    fn check_kernel_stack_guard(&self, pid: Pid) {
        self.0.process_as_ref(pid).check_kernel_stack_guard();
    }

    fn switch_kernel_stack(&self, next: Pid) {
        tss::set_kernel_stack_addr(self.0.process_as_ref(next).kernel_stack_bottom_addr());
    }

    fn context(&self, pid: Pid) -> *mut Context {
        self.0.process_as_ref(pid).context.get()
    }
}
impl<'a, const N: usize> From<&'a mut Manager<N>> for Switcher<'a, N> {
    fn from(manager: &'a mut Manager<N>) -> Self {
        Self(manager)
    }
}

// We use synronous sending not to overflow a message buffer by sending lots of messages to the
// same process.
struct Sender<'a, const N: usize> {
    manager: &'a mut Manager<N>,
    to: Pid,
    message: Message,
}
impl<'a, const N: usize> Sender<'a, N> {
    fn new(manager: &'a mut Manager<N>, to: Pid, message: Message) -> Self {
        Self {
            manager,
            to,
            message,
        }
    }

    fn send(mut self) {
        self.ensure_no_deadlocks();

        if self.is_receiver_waiting_message_from_me() {
            self.send_and_wake_receiver();
        } else {
            self.sleep();
        }
    }

    fn ensure_no_deadlocks(&self) {
        let mut proc_ptr = self.manager.process_as_ref(self.to);

        while let State::Sending { to, .. } = proc_ptr.state {
            if to == self.manager.running {
                panic!("A deadlock happened during sending a message.");
            }

            proc_ptr = self.manager.process_as_ref(to);
        }
    }

    fn is_receiver_waiting_message_from_me(&self) -> bool {
        match self.manager.process_as_ref(self.to).state {
            State::Receiving(ReceiveFrom::Any) => true,
            State::Receiving(ReceiveFrom::Pid(pid)) => self.manager.running == pid,
            _ => false,
        }
    }

    fn send_and_wake_receiver(&mut self) {
        let receiver = self.manager.process_as_mut(self.to);

        let message_buffer = receiver.message_buffer.as_mut();
        let message_buffer = message_buffer.expect("No message buffer.");

        message_buffer.write_volatile(self.message);

        receiver.message_buffer = None;

        self.manager.wake(self.to);
    }

    fn sleep(&mut self) {
        let sender = self.manager.process_as_mut(self.manager.running);

        sender.state = State::Sending {
            to: self.to,
            message: self.message,
        };

        let running = self.manager.running;
        let receiver = self.manager.process_as_mut(self.to);

        receiver.sending_to_this.push(running);
    }
}

struct Receiver<'a, const N: usize> {
    manager: &'a mut Manager<N>,
    from: ReceiveFrom,
    buffer: ReadWrite<Message>,
}
impl<'a, const N: usize> Receiver<'a, N> {
    fn new(manager: &'a mut Manager<N>, from: ReceiveFrom, buffer: ReadWrite<Message>) -> Self {
        Self {
            manager,
            from,
            buffer,
        }
    }

    fn receive(mut self) {
        self.ensure_no_deadlocks();

        if let Some(pid) = self.pop_sender_pid() {
            self.receive_and_wake_sender(pid);
        } else {
            self.sleep();
        }
    }

    fn ensure_no_deadlocks(&self) {
        let from = if let ReceiveFrom::Pid(pid) = self.from {
            pid
        } else {
            return;
        };

        let mut proc_ptr = self.manager.process_as_ref(from);

        while let State::Receiving(ReceiveFrom::Pid(from)) = proc_ptr.state {
            if from == self.manager.running {
                panic!("A deadlock happened during receiving a message.",);
            }

            proc_ptr = self.manager.process_as_ref(from);
        }
    }

    fn pop_sender_pid(&mut self) -> Option<Pid> {
        let pid_queue = &mut self
            .manager
            .process_as_mut(self.manager.running)
            .sending_to_this;

        let remove_index = match self.from {
            ReceiveFrom::Any => 0,
            ReceiveFrom::Pid(pid) => pid_queue.iter().position(|&p| p == pid)?,
        };

        pid_queue.pop_at(remove_index)
    }

    fn receive_and_wake_sender(&mut self, sender_pid: Pid) {
        let running = self.manager.running;

        let sender = self.manager.process_as_ref(sender_pid);
        let message = if let State::Sending { to, message } = sender.state {
            assert_eq!(
                to, running,
                "This process is not sending a message to this process."
            );

            message
        } else {
            panic!("The sender process is not sending a message.");
        };

        self.buffer.write_volatile(message);

        self.manager.wake(sender_pid);
    }

    fn sleep(self) {
        let receiver = self.manager.process_as_mut(self.manager.running);

        assert!(
            receiver.message_buffer.is_none(),
            "The message buffer is not empty."
        );

        receiver.state = State::Receiving(self.from);
        receiver.message_buffer = Some(self.buffer);
    }
}

// `Deque` is non-Copyable, so we cannot do `[Deque::new(); NUM_LEVEL]`. This is why we use the
// `Vec`, instead of an array.
struct RunnablePids<const NUM_LEVEL: usize>(Vec<Deque<Pid, MAX_PROCESS>, NUM_LEVEL>);
impl<const NUM_LEVEL: usize> RunnablePids<NUM_LEVEL> {
    const fn new() -> Self {
        Self(Vec::new())
    }

    fn init(&mut self) {
        for _ in 0..NUM_LEVEL {
            let r = self.0.push(Deque::new());

            // TODO: Use `expect`. Currently it is impossible because `Deque` does not implement
            // `Debug`.
            r.unwrap_or_else(|_| panic!("Failed to initialize the running pids queue."));
        }
    }

    fn push(&mut self, pid: Pid, priority: Priority) {
        let r = self.0[priority.as_usize()].push_back(pid);

        r.expect("The binary heap is full.");
    }

    fn pop(&mut self) -> Pid {
        self.0
            .iter_mut()
            .find_map(Deque::pop_front)
            .expect("No runnable PID.")
    }
}
