use {
    super::{context::Context, Pid, Process},
    crate::tss,
    heapless::Deque,
    spinning_top::{const_spinlock, Spinlock, SpinlockGuard},
};

const MAX_PROCESS: usize = 8;

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
        self.next_process_exists().then(|| self.switch())
    }

    fn next_process_exists(&self) -> bool {
        self.runnable_pids.front().is_some()
    }

    fn switch(&mut self) -> (*mut Context, *mut Context) {
        let next_pid = self.runnable_pids.pop_front();
        let next_pid = next_pid.expect("No next process.");

        self.switch_to(next_pid)
    }

    fn switch_to(&mut self, next: Pid) -> (*mut Context, *mut Context) {
        self.check_kernel_stack_guard(self.running);
        self.check_kernel_stack_guard(next);

        self.switch_kernel_stack(next);
        self.add_to_runnable_pid_queue(self.running);

        let current = self.running;

        self.running = next;

        (self.context(current), self.context(next))
    }

    fn check_kernel_stack_guard(&self, pid: Pid) {
        let proc = self.processes[pid.as_usize()].as_ref();
        let proc = proc.expect("No entry for the process.");

        proc.check_kernel_stack_guard();
    }

    fn switch_kernel_stack(&self, next: Pid) {
        let next_process = self.processes[next.as_usize()].as_ref();
        let next_process = next_process.expect("No entry for the next process.");

        tss::set_kernel_stack_addr(next_process.kernel_stack_bottom_addr());
    }

    fn context(&self, pid: Pid) -> *mut Context {
        let proc = self.processes[pid.as_usize()].as_ref();
        let proc = proc.expect("No entry for the process.");

        proc.context.get()
    }
}
