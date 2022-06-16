#![no_std]
#![no_main]

extern crate test_user_app as _;

#[no_mangle]
fn main() -> ! {
    syscalls::test_user_app_succeed();
}
