#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate rlibc as _;

mod acpi;
pub mod gdt;
pub mod idt;
pub mod mem;

use x86_64::structures::paging::Size4KiB;
use {core::panic::PanicInfo, qemu_print::qemu_println};

pub(crate) type NumOfPages<T = Size4KiB> = os_units::NumOfPages<T>;

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
pub fn exit_panic() -> ! {
    qemu::exit_failure();
}

#[cfg(not(test_on_qemu))]
pub fn exit_panic() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
