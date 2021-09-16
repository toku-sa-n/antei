pub(crate) mod manager;

use pid::Pid;

struct Process {
    pid: Pid,
}
impl Process {
    fn new(pid: Pid) -> Self {
        Self { pid }
    }
}
