#![no_std]
#![no_main]

// For `memcpy`.
extern crate rlibc as _;

use core::panic::PanicInfo;
use stable_uefi as uefi;

#[no_mangle]
pub extern "win64" fn efi_main(_: uefi::Handle, mut st: uefi::SystemTable) -> ! {
    let mut stdout = st.con_out();
    let string = "hello world".as_bytes();
    let mut buf = [0_u16; 32];

    for i in 0..string.len() {
        buf[i] = string[i].into();
    }

    let s = stdout.reset_without_extension();
    s.expect("Failed to reset the console.");
    loop {
        let s = stdout.output_string(&mut buf);
        s.expect("Failed to print a string.");
    }
}

#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
