#![no_std]

use core::panic;

fn main() {}

#[panic_handler]
fn panic(_: &panic::PanicInfo) -> ! {
    loop {}
}
