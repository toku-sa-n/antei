#![no_std]

extern crate rlibc as _;

mod mount;
mod process;

pub fn init() {
    process::manager::init();
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
