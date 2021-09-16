#![no_std]
#![no_main]

extern crate vfs as _;

#[no_mangle]
fn main() -> ! {
    vfs::init();

    loop {
        core::hint::spin_loop();
    }
}
