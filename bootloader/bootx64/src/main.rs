#![no_std]
#![no_main]

// For `memcpy`.
extern crate rlibc as _;

use bootx64::gop;
use bootx64::mem;
use bootx64::{fs, uefi_println};

#[no_mangle]
pub extern "win64" fn efi_main(_: uefi_wrapper::Handle, mut st: bootx64::SystemTable) -> ! {
    let resolution = gop::set_preferred_resolution(&mut st);
    uefi_println!(&mut st, "GOP info: {:?}", resolution,);

    let bytes = fs::locate(&mut st, "kernel");
    uefi_println!(&mut st, "{:X?}", &bytes[0..8]);

    let mem_map_size = mem::get_memory_map_size(&mut st);
    uefi_println!(&mut st, "Memory map size: {}", mem_map_size);

    loop {
        x86_64::instructions::hlt();
    }
}
