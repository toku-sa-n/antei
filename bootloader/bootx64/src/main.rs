#![no_std]
#![no_main]

// For `memcpy`.
extern crate rlibc as _;

use core::panic::PanicInfo;
use log::error;
use log::info;

#[no_mangle]
pub extern "win64" fn efi_main(h: bootx64::Handle, st: bootx64::SystemTable) -> ! {
    bootx64::init(h, st);

    loop {
        info!("hello world");
    }
}

#[panic_handler]
fn panic(i: &PanicInfo<'_>) -> ! {
    error!("{}", i);

    loop {
        x86_64::instructions::hlt();
    }
}
