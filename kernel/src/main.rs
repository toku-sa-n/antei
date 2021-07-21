#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate kernel as _;

use {
    boot_info::BootInfo,
    kernel::{fini, gdt, idt, mem},
};

#[no_mangle]
extern "sysv64" fn main(mut boot_info: BootInfo) {
    boot_info.validate();

    mem::phys::init(boot_info.mmap_mut().as_slice_mut());

    gdt::init();
    idt::init();

    fini();
}
