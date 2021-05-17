#![no_std]
#![no_main]

// For `memcpy`.
extern crate rlibc as _;

use log::info;

#[no_mangle]
pub extern "win64" fn efi_main(h: bootx64::Handle, st: bootx64::SystemTable) -> ! {
    bootx64::init(h, st);

    loop {
        info!("hello world");
    }
}
