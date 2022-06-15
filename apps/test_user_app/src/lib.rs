#![no_std]

extern crate rlibc as _;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    syscalls::test_user_app_failed();
}
