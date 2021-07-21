#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate rlibc as _;

pub mod gdt;
pub mod idt;
pub mod mem;

use {core::panic::PanicInfo, qemu_print::qemu_println, x86_64::structures::paging::Size4KiB};

pub(crate) type NumOfPages<T = Size4KiB> = os_units::NumOfPages<T>;

#[cfg(feature = "test_on_qemu")]
pub fn fini() -> ! {
    qemu::exit_success();
}

#[cfg(not(feature = "test_on_qemu"))]
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

#[cfg(feature = "test_on_qemu")]
pub fn exit_panic() -> ! {
    qemu::exit_failure();
}

#[cfg(not(feature = "test_on_qemu"))]
pub fn exit_panic() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
