#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

use bootx64::{kernel, paging, rsdp, stack};

#[no_mangle]
extern "win64" fn efi_main(h: uefi_wrapper::Handle, mut st: bootx64::SystemTable) -> ! {
    let binary = kernel::locate(&mut st);

    let rsdp = rsdp::get(&st);

    let mmap = bootx64::exit_boot_services_and_return_mmap(h, st);

    // SAFETY: Yes, the addresses are the same.
    unsafe { paging::enable_recursive_paging() };

    // SAFETY: Yes, the recursive paging is enabled and there are no references to one of all
    // working page tables.
    unsafe { stack::allocate(mmap) };

    // SAFETY: Yes, the recursive paging is enabled and there are no references to the PML4.
    unsafe { kernel::load_and_jump(binary, mmap, rsdp) };
}
