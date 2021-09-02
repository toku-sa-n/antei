#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

use bootx64::{initrd, kernel, paging, rsdp, stack};

#[no_mangle]
fn efi_main(h: uefi::Handle, mut st: bootx64::SystemTable) -> ! {
    let kernel_binary = kernel::locate(&mut st);

    let initrd_binary = initrd::locate(&mut st);

    let rsdp = rsdp::get(&st);

    let mmap = bootx64::exit_boot_services_and_return_mmap(h, st);

    // SAFETY: Yes, the addresses are the same.
    unsafe {
        paging::enable_recursive_paging();
    }

    // SAFETY: Yes, the recursive paging is enabled and there are no references to one of all
    // working page tables.
    unsafe {
        stack::allocate(mmap);
    }

    // SAFETY: The virtual address `0xff7f_bfdf_e000` points to the current working PML4 and any
    // references do not point to one of all working page tables.
    unsafe {
        initrd::map_and_load(initrd_binary, mmap);
    }

    // SAFETY: Yes, the recursive paging is enabled and there are no references to the PML4.
    unsafe {
        kernel::load_and_jump(kernel_binary, mmap, rsdp);
    }
}
