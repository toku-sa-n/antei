#![no_std]
#![no_main]

extern crate shell as _;

#[no_mangle]
fn main() -> ! {
    loop {
        core::hint::spin_loop();
    }
}
