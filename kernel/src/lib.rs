#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

mod gdt;

use core::panic;

#[panic_handler]
fn panic(_: &panic::PanicInfo<'_>) -> ! {
    loop {}
}
