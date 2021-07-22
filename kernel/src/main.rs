#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate kernel as _;

use {
    aligned_ptr::ptr,
    boot_info::BootInfo,
    kernel::{fini, gdt, idt, mem},
};

/// # Safety
///
/// `boot_info` must be dereferenceable.
#[no_mangle]
unsafe extern "sysv64" fn main(boot_info: *mut BootInfo) {
    // SAFETY: `boot_info` is the pointer passed from the bootloader. w
    let mut boot_info = unsafe { ptr::get(boot_info) };
    boot_info.validate();

    mem::phys::init(boot_info.mmap_mut().as_slice_mut());

    gdt::init();
    idt::init();

    fini();
}
