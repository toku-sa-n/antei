#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate rlibc as _;

mod gdt;
mod interrupt;
mod libc;
mod log;
mod process;
mod sysproc;
#[cfg(test_on_qemu)]
mod tests;
mod timer;
mod tss;

use {boot_info::BootInfo, core::panic::PanicInfo, interrupt::idt, qemu_print::qemu_println};

pub fn init(boot_info: BootInfo) {
    // SAFETY: `boot_info` is the pointer passed from the bootloader. w
    boot_info.validate();

    log::init();

    // SAFETY: The recursive address is accessible and there are no references to the current
    // working PML4.
    unsafe {
        vm::init(boot_info.mmap().as_slice());
    }

    gdt::init();
    idt::init();

    pic::init();

    // SAFETY: `boot_info.rsdp()` returns the address of RSDP that is fetched from the
    // configuration table of UEFI's system table.
    unsafe {
        timer::init(boot_info.rsdp());
    }

    process::init();
}

pub fn idle() -> ! {
    loop {
        x86_64::instructions::interrupts::enable_and_hlt();
    }
}

#[panic_handler]
fn panic(i: &PanicInfo<'_>) -> ! {
    x86_64::instructions::interrupts::disable();

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
