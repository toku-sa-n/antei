#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
