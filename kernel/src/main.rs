#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate kernel as _;

use kernel::{fini, gdt};

#[no_mangle]
fn main() {
    gdt::init();

    fini();
}
