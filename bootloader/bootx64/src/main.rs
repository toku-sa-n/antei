#![no_std]
#![no_main]

// For `memcpy`.
extern crate rlibc as _;

use core::panic::PanicInfo;
use log::error;
use log::info;
use uefi_global as uefi;

#[no_mangle]
pub extern "win64" fn efi_main(h: uefi::Handle, st: uefi::SystemTable) -> ! {
    uefi_global::init(h, st);

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
