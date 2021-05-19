#![no_std]
#![no_main]

#[no_mangle]
fn main() {
    unsafe {
        *(0x334 as *mut u8) = 3_u8;
    }
}
