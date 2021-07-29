#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate kernel as _;

use {
    aligned_ptr::ptr,
    boot_info::BootInfo,
    kernel::{fini, init},
};

/// # Safety
///
/// `boot_info` must be dereferenceable.
#[no_mangle]
unsafe extern "sysv64" fn main(boot_info: *mut BootInfo) {
    // SAFETY: `boot_info` must be dereferenceable.
    init(unsafe { ptr::as_mut(boot_info) });

    fini();
}
