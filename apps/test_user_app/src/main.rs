#![no_std]
#![no_main]

extern crate test_user_app as _;

#[no_mangle]
fn main() -> ! {
    syscalls::write(core::str::from_utf8(&[b'a'; 256]).unwrap());
    syscalls::test_user_app_succeed();
}
