#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate kernel as _;

use kernel::gdt;
use x86_64::instructions::hlt;

#[no_mangle]
fn main() {
    gdt::init();

    loop {
        hlt();
    }
}
