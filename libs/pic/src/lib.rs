#![no_std]

use {
    pic8259::ChainedPics,
    spinning_top::{const_spinlock, Spinlock},
};

// SAFETY: The vectors are correct.
static PICS: Spinlock<ChainedPics> = const_spinlock(unsafe { ChainedPics::new(0x20, 0x28) });

pub fn init() {
    let pics = PICS.try_lock();
    let mut pics = pics.expect("Failed to lock PIC.");

    // SAFETY: Disabling PIC does not violate memory safety.
    unsafe {
        pics.initialize();
        pics.disable();
    }
}
