use {
    super::{context::Context, Pid, Priority, Process, State, LEAST_PRIORITY_LEVEL, MAX_PROCESS},
    crate::tss,
    heapless::{Deque, Vec},
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

fn lock<'a>() -> SpinlockGuard<'a, Manager<MAX_PROCESS>> {
    let m = MANAGER.try_lock();

    m.expect("Failed to lock the process manager.")
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
