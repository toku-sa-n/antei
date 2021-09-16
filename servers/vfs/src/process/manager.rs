use {
    super::Process,
    config::MAX_PID,
    core::convert::TryInto,
    pid::{predefined, Pid},
    spinning_top::{const_spinlock, Spinlock, SpinlockGuard},
};

static MANAGER: Spinlock<Manager<MAX_PID>> = const_spinlock(Manager::new());

pub(crate) fn init() {
    const PROC_INFO: u64 = 1;

    let mut message;
    while {
        message = ipc::receive(predefined::PM.into());

        message.body.0 == PROC_INFO
    } {
        let pid = Pid::new(message.body.1.try_into().unwrap());

        let process = Process::new(pid);

        lock().add(process);
    }
}

fn lock<'a>() -> SpinlockGuard<'a, Manager<MAX_PID>> {
    MANAGER.try_lock().expect("Failed to lock `MANAGER`.")
}

struct Manager<const N: usize> {
    processes: [Option<Process>; N],
}
impl<const N: usize> Manager<N> {
    const fn new() -> Self {
        const NULL_SLOT: Option<Process> = None;

        Self {
            processes: [NULL_SLOT; N],
        }
    }

    fn add(&mut self, process: Process) {
        let pid = process.pid;

        let slot = &mut self.processes[pid.as_usize()];

        if slot.is_none() {
            *slot = Some(process);
        } else {
            panic!("Duplicated process with {}", pid);
        }
    }
}
