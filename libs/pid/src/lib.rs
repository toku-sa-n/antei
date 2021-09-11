#![no_std]

use {
    core::{
        convert::{TryFrom, TryInto},
        fmt,
    },
    posix::sys::types::Pid as PosixPid,
};

// We use `usize` because it is more valuable than `PosixPid`. After all, we can use it as an index
// of an array.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pid(usize);
impl Pid {
    #[must_use]
    pub const fn new(pid: usize) -> Self {
        Self(pid)
    }

    #[must_use]
    pub const fn as_usize(self) -> usize {
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
impl TryFrom<PosixPid> for Pid {
    type Error = NegativePid;

    fn try_from(pid: PosixPid) -> Result<Self, Self::Error> {
        pid.try_into().map(Self).map_err(|_| NegativePid(pid))
    }
}
impl From<Pid> for PosixPid {
    fn from(pid: Pid) -> Self {
        pid.as_usize()
            .try_into()
            .unwrap_or_else(|_| unreachable!("PID must be less than or equal to `PosixPid::MAX`."))
    }
}
impl fmt::Display for Pid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PID {}", self.as_usize())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NegativePid(PosixPid);
