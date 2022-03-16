use {
    super::Message,
    core::{arch::asm, convert::TryInto, mem::MaybeUninit},
    num_derive::FromPrimitive,
    pid::Pid,
    posix::sys::types::Pid as PosixPid,
};

fn execute_syscall(index: Ty, a1: u64, a2: u64) -> u64 {
    let r: u64;

    unsafe {
        asm!("syscall",
        inout("rdi") index as u64 => _,
        inout("rsi") a1 => _,
        inout("rdx") a2 => _,
        out("rax") r,
        out("rcx") _,
        out("r8") _,
        out("r9") _,
        out("r10") _,
        out("r11") _,
        out("xmm0") _,
        out("xmm1") _,
        out("xmm2") _,
        out("xmm3") _,
        out("xmm4") _,
        out("xmm5") _,
        out("xmm6") _,
        out("xmm7") _,
        out("xmm8") _,
        out("xmm9") _,
        out("xmm10") _,
        out("xmm11") _,
        out("xmm12") _,
        out("xmm13") _,
        out("xmm14") _,
        out("xmm15") _);
    }

    r
}

/// # Panics
///
/// This function panics if there is no process with PID `to`.
pub fn send(to: Pid, message: Message) {
    try_send(to, message).expect("Failed to send a message.");
}

/// # Errors
///
/// This function returns an error if there is no process with PID `to`.
#[cfg_attr(target_pointer_width = "64", allow(clippy::missing_panics_doc))]
pub fn try_send(to: Pid, message: Message) -> Result<(), Error> {
    let message: *const _ = &message;

    if execute_syscall(Ty::Send, to.as_usize().try_into().unwrap(), message as _) == 0 {
        Ok(())
    } else {
        Err(Error)
    }
}

/// # Panics
///
/// This method panics if there is no process with PID `from` specifies.
#[must_use]
pub fn receive(from: ReceiveFrom) -> Message {
    try_receive(from).expect("Failed to receive a message.")
}

/// # Errors
///
/// This function returns an error if there is no process with PID `from` specifies.
pub fn try_receive(from: ReceiveFrom) -> Result<Message, Error> {
    let mut m = MaybeUninit::uninit();

    let from = match from {
        ReceiveFrom::Any => -1,
        ReceiveFrom::Pid(pid) => PosixPid::from(pid),
    };

    // We cannot use `try_into` because it returns an error if the value is negative while the
    // negative PID is valid here because it means the sender's PID is unspecified. Also, the
    // sign information will not be lost as the kernel casts it to `i32` again.
    #[allow(clippy::cast_sign_loss)]
    if execute_syscall(Ty::Receive, from as _, m.as_mut_ptr() as _) == 0 {
        Ok(unsafe { m.assume_init() })
    } else {
        Err(Error)
    }
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
impl From<Pid> for ReceiveFrom {
    fn from(pid: Pid) -> Self {
        Self::Pid(pid)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Error;
