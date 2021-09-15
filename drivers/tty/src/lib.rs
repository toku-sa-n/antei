#![no_std]

extern crate rlibc as _;

mod font;
mod vram;
mod writer;

#[doc(hidden)]
pub use writer::_print;

pub fn init() {
    let screen_info = syscalls::get_screen_info();

    vram::init(screen_info);
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
