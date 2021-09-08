use {
    super::{manager, MAX_PROCESS},
    core::{
        convert::{TryFrom, TryInto},
        fmt,
        num::TryFromIntError,
    },
    posix::sys::types::Pid as PosixPid,
};

// We use `usize` because it is more valuable than `PosixPid`. After all, we can use it as an index
// of an array.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Pid(usize);
impl Pid {
    pub(crate) const fn new(pid: usize) -> Self {
        Self(pid)
    }

    pub(super) const fn as_usize(self) -> usize {
        self.0
    }

    pub(super) fn generate() -> Option<Self> {
        (0..MAX_PROCESS).find_map(|i| (!manager::process_exists(i.into())).then(|| Self(i)))
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
impl TryFrom<PosixPid> for Pid {
    type Error = NegativePid;

    fn try_from(pid: PosixPid) -> Result<Self, Self::Error> {
        pid.try_into().map(Self).map_err(|_| NegativePid(pid))
    }
}
impl TryFrom<Pid> for PosixPid {
    type Error = TryFromIntError;
    fn try_from(pid: Pid) -> Result<Self, Self::Error> {
        pid.as_usize().try_into()
    }
}
impl fmt::Display for Pid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PID {}", self.as_usize())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct NegativePid(PosixPid);
