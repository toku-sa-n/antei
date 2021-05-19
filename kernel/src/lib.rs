#![no_std]

use core::panic;

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
