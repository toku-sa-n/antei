#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate kernel as _;

use kernel::gdt;

#[no_mangle]
fn main() {
    gdt::init();

    unsafe {
        loop {
            *(0x334 as *mut u8) = 3_u8;
        }
    }
}
