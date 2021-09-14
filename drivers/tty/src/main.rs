#![no_std]
#![no_main]

extern crate tty as _;

#[no_mangle]
fn main() -> ! {
    let _ = syscalls::get_screen_info();

    loop {
        core::hint::spin_loop();
    }
}
