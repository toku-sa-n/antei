#![no_std]
#![no_main]

extern crate tty as _;

#[no_mangle]
fn main() -> ! {
    syscalls::get_screen_info();
    loop {}
}
