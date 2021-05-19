#![no_std]
#![no_main]

extern crate kernel as _;

#[no_mangle]
fn main() {
    unsafe {
        *(0x334 as *mut u8) = 3_u8;
    }
}
