#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate kernel as _;

use {
    aligned_ptr::ptr,
    boot_info::BootInfo,
    kernel::{idle, init},
};

compile_error!("Clippy test.");

/// # Safety
///
/// `boot_info` must be dereferenceable.
#[no_mangle]
unsafe extern "sysv64" fn main(boot_info: *mut BootInfo) {
    // SAFETY: `boot_info` must be dereferenceable.
    init(unsafe { ptr::get(boot_info) });

    idle();
}
