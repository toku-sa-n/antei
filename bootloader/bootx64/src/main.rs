#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

// For `memcpy`.
extern crate rlibc as _;

use bootx64::elf;
use bootx64::gop;
use bootx64::paging;
use bootx64::{fs, uefi_println};

#[no_mangle]
extern "win64" fn efi_main(h: uefi_wrapper::Handle, mut st: bootx64::SystemTable) -> ! {
    let resolution = gop::set_preferred_resolution(&mut st);
    uefi_println!(&mut st, "GOP info: {:?}", resolution);

    let bytes = fs::locate(&mut st, "kernel");
    uefi_println!(&mut st, "{:X?}", &bytes[0..8]);

    let mmap = bootx64::exit_boot_services_and_return_mmap(h, st);

    // SAFETY: Yes, the addresses are the same.
    unsafe { paging::enable_recursive_paging() };

    // SAFETY: Yes, the recursive paging is enabled and there are no references to the PML4.
    let entry = unsafe { elf::load(bytes, mmap) };
    assert!(!entry.is_null(), "The entry address is null.");

    // SAFETY: Safe as described in
    // https://rust-lang.github.io/unsafe-code-guidelines/layout/function-pointers.html#representation.
    let entry: fn() -> ! = unsafe { core::mem::transmute(entry) };

    (entry)()
}
