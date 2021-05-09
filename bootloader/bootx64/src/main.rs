#![no_std]
#![no_main]

// For `memcpy`.
extern crate rlibc as _;

use core::panic::PanicInfo;
use log::info;
use uefi_global as uefi;
use uefi_global::edid;

#[no_mangle]
pub extern "win64" fn efi_main(h: uefi::Handle, st: uefi::SystemTable) -> ! {
    uefi_global::init(h, st);

    info!("Preferred resolution: {:?}", edid::preferred_resolution());

    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
