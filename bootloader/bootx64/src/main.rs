#![no_std]
#![no_main]

// For `memcpy`.
extern crate rlibc as _;

use bootx64::fs;
use bootx64::gop;
use log::info;

#[no_mangle]
pub extern "win64" fn efi_main(_: uefi_wrapper::Handle, mut st: uefi_wrapper::SystemTable) -> ! {
    info!("GOP info: {:?}", gop::set_preferred_resolution(&mut st));
    let bytes = fs::locate(&mut st, "kernel");
    info!("{:X?}", &bytes[0..8]);

    loop {
        x86_64::instructions::hlt();
    }
}
