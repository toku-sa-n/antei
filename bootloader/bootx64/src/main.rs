#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

// For `memcpy`.
extern crate rlibc as _;

use bootx64::elf;
use bootx64::gop;
use bootx64::{fs, uefi_println};

#[no_mangle]
pub extern "win64" fn efi_main(h: uefi_wrapper::Handle, mut st: bootx64::SystemTable) -> ! {
    let resolution = gop::set_preferred_resolution(&mut st);
    uefi_println!(&mut st, "GOP info: {:?}", resolution,);

    let bytes = fs::locate(&mut st, "kernel");
    uefi_println!(&mut st, "{:X?}", &bytes[0..8]);

    let mmap = bootx64::exit_boot_services_and_return_mmap(h, st);

    elf::load(bytes, mmap);

    loop {
        x86_64::instructions::hlt();
    }
}
