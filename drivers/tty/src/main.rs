#![no_std]
#![no_main]

extern crate tty as _;

#[no_mangle]
fn main() -> ! {
    tty::init();

    tty::main_loop();
}
