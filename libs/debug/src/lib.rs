#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

use core::arch::asm;

pub fn stop() -> ! {
    // SAFETY: This code just loops forever.
    //
    // To avoid a bug, we use 3 instead of 0 for the label. See: https://github.com/rust-lang/rust/issues/74558
    unsafe {
        asm!(
            "mov rax, 0x55aa55aa55aa55aa
            3:
            jmp 3b",
            options(noreturn, nomem)
        );
    }
}
