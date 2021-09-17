#![no_std]
#![no_main]

extern crate xhci as _;

#[no_mangle]
fn main() -> ! {
    loop {
        core::hint::spin_loop();
    }
}
