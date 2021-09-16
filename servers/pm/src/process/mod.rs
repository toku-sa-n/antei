pub(crate) mod manager;

use pid::Pid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Process {
    pid: Pid,
}
impl Process {
    fn new(pid: Pid) -> Self {
        Self { pid }
    }
}
