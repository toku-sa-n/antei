#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

use core::panic;

#[panic_handler]
fn panic(_: &panic::PanicInfo<'_>) -> ! {
    loop {}
}
