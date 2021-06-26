#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate kernel as _;

#[no_mangle]
fn main() {
    unsafe {
        loop {
            *(0x334 as *mut u8) = 3_u8;
        }
    }
}
