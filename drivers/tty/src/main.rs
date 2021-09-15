#![no_std]
#![no_main]

extern crate tty as _;

use tty::println;

#[no_mangle]
fn main() -> ! {
    tty::init();

    println!("hello, world");

    loop {
        core::hint::spin_loop();
    }
}
