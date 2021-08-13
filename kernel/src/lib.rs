#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate rlibc as _;

mod gdt;
mod idt;
mod log;
mod timer;

use {boot_info::BootInfo, core::panic::PanicInfo, qemu_print::qemu_println};

pub fn init(boot_info: BootInfo) {
    // SAFETY: `boot_info` is the pointer passed from the bootloader. w
    boot_info.validate();

    log::init();

    // SAFETY: The recursive address is accessible and there are no references to the current
    // working PML4.
    unsafe {
        kernel_mem::init(boot_info.mmap().as_slice());
    }

    gdt::init();
    idt::init();

    // SAFETY: `boot_info.rsdp()` returns the address of RSDP that is fetched from the
    // configuration table of UEFI's system table.
    unsafe {
        timer::init(boot_info.rsdp());
    }
}

#[cfg(test_on_qemu)]
pub fn fini() -> ! {
    qemu::exit_success();
}

#[cfg(not(test_on_qemu))]
pub fn fini() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(i: &PanicInfo<'_>) -> ! {
    qemu_println!("{}", i);

    exit_panic();
}

#[cfg(test_on_qemu)]
fn exit_panic() -> ! {
    qemu::exit_failure();
}

#[cfg(not(test_on_qemu))]
fn exit_panic() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
