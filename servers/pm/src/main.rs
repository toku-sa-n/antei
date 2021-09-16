#![no_std]
#![no_main]

extern crate pm as _;

#[no_mangle]
fn main() -> ! {
    pm::init();

    loop {
        syscalls::write("PM server.\n");
    }
}
