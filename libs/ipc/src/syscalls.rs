use {
    super::Message,
    core::{convert::TryInto, mem::MaybeUninit},
    num_derive::FromPrimitive,
};

extern "sysv64" {
    fn execute_syscall(index: Ty, a1: u64, a2: u64);
}

pub fn send(to: usize, message: Message) {
    let message: *const _ = &message;

    unsafe {
        execute_syscall(Ty::Send, to.try_into().unwrap(), message as _);
    }
}

pub fn receive(from: ReceiveFrom) -> Message {
    let mut m = MaybeUninit::uninit();

    let from = match from {
        ReceiveFrom::Any => usize::MAX,
        ReceiveFrom::Pid(pid) => {
            assert_ne!(pid, usize::MAX);

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
    Pid(usize),
}
