#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate kernel as _;

use {
    boot_info::BootInfo,
    kernel::{fini, gdt, idt},
};

#[no_mangle]
extern "sysv64" fn main(boot_info: BootInfo) {
    boot_info.check_header_and_footer();

    gdt::init();
    idt::init();

    fini();
}
