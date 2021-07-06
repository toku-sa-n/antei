#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate rlibc as _;

pub mod gdt;

use core::panic;

#[panic_handler]
fn panic(_: &panic::PanicInfo<'_>) -> ! {
    loop {}
}
