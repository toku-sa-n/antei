#![no_std]
#![no_main]

use core::panic;

#[no_mangle]
fn main() {
    unsafe {
        *(0x334 as *mut u8) = 3_u8;
    }
}

#[panic_handler]
fn panic(_: &panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
fn __libc_start_main(main: fn() -> isize) {
    main();
}

#[no_mangle]
fn __libc_csu_init() {}

#[no_mangle]
fn __libc_csu_fini() {}
