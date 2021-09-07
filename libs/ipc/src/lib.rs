#![no_std]

use {
    core::{convert::TryInto, mem::MaybeUninit},
    message::Message,
};

extern "sysv64" {
    fn asm_syscall(index: u64, a1: u64, a2: u64);
}

pub fn send(to: usize, message: Message) {
    // SAFETY: The system call index and the address to the message buffer are correct.
    unsafe {
        asm_syscall(0, to.try_into().unwrap(), &message as *const _ as u64);
    }
}

pub fn receive(from: usize) -> Message {
    let mut message = MaybeUninit::uninit();

    // SAFETY: The system call index and the pointer to the message buffer are correct.
    unsafe {
        asm_syscall(1, from.try_into().unwrap(), message.as_mut_ptr() as _);
    }

    // SAFETY: The previous `asm_syscall` initializes `message`.
    unsafe { message.assume_init() }
}
