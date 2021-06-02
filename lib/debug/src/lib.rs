#![no_std]

extern "C" {
    fn asm_stop();
}

pub fn stop() {
    // SAFETY: `asm_stop` just loops infinitely.
    unsafe { asm_stop() }
}
