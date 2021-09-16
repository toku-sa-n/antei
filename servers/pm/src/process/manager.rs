use {
    super::Process,
    config::MAX_PID,
    core::convert::TryInto,
    pid::Pid,
    spinning_top::{const_spinlock, Spinlock, SpinlockGuard},
};

static MANAGER: Spinlock<Manager<MAX_PID>> = const_spinlock(Manager::new());

pub(crate) fn init() {
    while let Some(message) = syscalls::pm_syncs_with_kernel() {
        let pid = Pid::new(message.body.1.try_into().unwrap());

        let process = Process::new(pid);

        lock().add(process);
    }
}

fn lock<'a>() -> SpinlockGuard<'a, Manager<MAX_PID>> {
    MANAGER.try_lock().expect("Failed to lock `MANAGER`")
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Manager<const N: usize> {
    processes: [Option<Process>; N],
}
impl<const N: usize> Manager<N> {
    const fn new() -> Self {
        const NULL_PROC: Option<Process> = None;

        Self {
            processes: [NULL_PROC; N],
        }
    }

    fn add(&mut self, process: Process) {
        let pid = process.pid;

        let slot = &mut self.processes[pid.as_usize()];

        if slot.is_some() {
            panic!("Duplicated proces with {}", pid);
        } else {
            *slot = Some(process);
        }
    }
}
