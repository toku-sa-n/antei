#![no_std]

pub mod fs;
pub mod gop;
pub mod io;
pub mod panic;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
