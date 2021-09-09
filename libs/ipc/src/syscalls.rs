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
/// This function panics if `to <= 0`.
pub fn send(to: Pid, message: Message) {
    let to = to.try_into();
    let to = to.expect("Invalid PID.");

    let message: *const _ = &message;

    unsafe {
        execute_syscall(Ty::Send, to, message as _);
    }
}

/// # Panics
///
/// This function panics if `from` specifies a negative PID.
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
        // We cannot use `try_into` because it returns an error if the value is negative while the
        // negative PID is valid here because it means the sender's PID is unspecified. Also, the
        // sign information will not be lost as the kernel casts it to `i32` again.
        #[allow(clippy::cast_sign_loss)]
        execute_syscall(Ty::Receive, from as _, m.as_mut_ptr() as _);
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
