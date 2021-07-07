#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate kernel as _;

use kernel::gdt;
use qemu_print::qemu_println;

#[no_mangle]
fn main() {
    qemu_println!("Hello world.");

    gdt::init();

    unsafe {
        loop {
            *(0x334 as *mut u8) = 3_u8;
        }
    }
}
