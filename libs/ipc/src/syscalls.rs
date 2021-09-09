use {
    super::Message,
    core::{convert::TryInto, mem::MaybeUninit},
    num_derive::FromPrimitive,
    posix::sys::types::Pid,
};

extern "sysv64" {
    fn execute_syscall(index: Ty, a1: u64, a2: u64);
}

/// # Panics
///
/// This method panics if `to <= 0`.
pub fn send(to: Pid, message: Message) {
    assert!(to > 0, "Invalid PID.");

    let message: *const _ = &message;

    unsafe {
        execute_syscall(Ty::Send, to.try_into().unwrap(), message as _);
    }
}

/// # Panics
///
/// This method panics if `from` specifies a negative PID.
#[must_use]
pub fn receive(from: ReceiveFrom) -> Message {
    let mut m = MaybeUninit::uninit();

    let from = match from {
        ReceiveFrom::Any => -1,
        ReceiveFrom::Pid(pid) => {
            assert!(pid > 0, "Invalid PID.");

            pid
        }
    };

    unsafe {
        execute_syscall(Ty::Receive, from.try_into().unwrap(), m.as_mut_ptr() as _);
    }

    unsafe { m.assume_init() }
}

#[repr(u64)]
#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ty {
    Send,
    Receive,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReceiveFrom {
    Any,
    Pid(Pid),
}
