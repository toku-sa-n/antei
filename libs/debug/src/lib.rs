#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

extern "C" {
    fn asm_stop() -> !;
}

pub fn stop() -> ! {
    // SAFETY: `asm_stop` just loops infinitely.
    unsafe { asm_stop() }
}
