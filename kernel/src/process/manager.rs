use {
    super::{context::Context, Pid, Process, State, MAX_PROCESS},
    crate::tss,
    heapless::Deque,
    spinning_top::{const_spinlock, Spinlock, SpinlockGuard},
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

pub(super) fn process_exists(pid: Pid) -> bool {
    lock().exists(pid)
}

pub(super) fn add(p: Process) {
    lock().add(p);
}

pub(super) fn add_idle() {
    lock().add_idle();
}

fn lock<'a>() -> SpinlockGuard<'a, Manager<MAX_PROCESS>> {
    let m = MANAGER.try_lock();

    m.expect("Failed to lock the process manager.")
}

struct Manager<const N: usize> {
    processes: [Option<Process>; N],

    // The running PID is not contained.
    runnable_pids: Deque<Pid, N>,

    running: Pid,
}
impl<const N: usize> Manager<N> {
    const fn new() -> Self {
        const UNUSED_PROCESS_ENTRY: Option<Process> = None;

        Self {
            processes: [UNUSED_PROCESS_ENTRY; N],
            runnable_pids: Deque::new(),
            running: Pid::new(0),
        }
    }

    fn add_idle(&mut self) {
        let idle = super::Process::idle();

        assert_eq!(idle.pid, Pid::new(0), "Wrong PID for the idle process.");

        tss::set_kernel_stack_addr(idle.kernel_stack_bottom_addr());

        self.add_to_process_collection(idle);
    }

    fn add(&mut self, p: Process) {
        self.add_to_runnable_pid_queue(p.pid);
        self.add_to_process_collection(p);
    }

    fn add_to_runnable_pid_queue(&mut self, pid: Pid) {
        let r = self.runnable_pids.push_back(pid);
        r.expect("Too many runnable pids.");
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
        let next = self.update_runnable_pids_and_return_next_pid();

        (self.running != next).then(|| self.switch_to(next))
    }

    fn update_runnable_pids_and_return_next_pid(&mut self) -> Pid {
        if self.process_as_ref(self.running).state == State::Running {
            self.push_current_process_as_runnable();
        }

        let r = self.runnable_pids.pop_front();
        r.expect("No runnable PID.")
    }

    fn push_current_process_as_runnable(&mut self) {
        self.process_as_mut(self.running).state = State::Runnable;

        let r = self.runnable_pids.push_back(self.running);
        r.expect("The runnable pid queue is full.");
    }

    fn switch_to(&mut self, next: Pid) -> (*mut Context, *mut Context) {
        self.check_kernel_stack_guard(self.running);
        self.check_kernel_stack_guard(next);

        self.switch_kernel_stack(next);

        let current = self.running;

        self.running = next;
        self.process_as_mut(next).state = State::Running;

        (self.context(current), self.context(next))
    }

    fn check_kernel_stack_guard(&self, pid: Pid) {
        self.process_as_ref(pid).check_kernel_stack_guard();
    }

    fn switch_kernel_stack(&self, next: Pid) {
        tss::set_kernel_stack_addr(self.process_as_ref(next).kernel_stack_bottom_addr());
    }

    fn context(&self, pid: Pid) -> *mut Context {
        self.process_as_ref(pid).context.get()
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
