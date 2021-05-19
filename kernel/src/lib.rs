#![no_std]

use core::panic;

#[panic_handler]
fn panic(_: &panic::PanicInfo<'_>) -> ! {
    loop {}
}
