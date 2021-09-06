use {
    super::{manager, MAX_PROCESS},
    core::fmt,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Pid(usize);
impl Pid {
    pub(crate) const fn new(pid: usize) -> Self {
        Self(pid)
    }

    pub(super) fn generate() -> Option<Self> {
        (0..MAX_PROCESS).find_map(|i| (!manager::process_exists(i.into())).then(|| Self(i)))
    }

    pub(super) fn as_usize(self) -> usize {
        self.0
    }
}
impl From<Pid> for usize {
    fn from(pid: Pid) -> Self {
        pid.0
    }
}
impl From<usize> for Pid {
    fn from(pid: usize) -> Self {
        Pid(pid)
    }
}
impl fmt::Display for Pid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PID {}", self.as_usize())
    }
}
