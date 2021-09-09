use posix::sys::types::Pid;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    NoSuchProcess(Pid),
    SenderNotFound,
    Deadlock,
}
