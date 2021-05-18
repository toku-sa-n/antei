#![no_std]
#![no_main]

// For `memcpy`.
extern crate rlibc as _;

use bootx64::gop;
use log::info;

#[no_mangle]
pub extern "win64" fn efi_main(h: bootx64::Handle, st: bootx64::SystemTable) -> ! {
    bootx64::init(h, st);

    info!("Preferred resolution: {:?}", gop::preferred_resolution());

    loop {
        x86_64::instructions::hlt();
    }
}
