#![no_std]
#![no_main]

// For `memcpy`.
extern crate rlibc as _;

use core::fmt::Write;
use core::panic::PanicInfo;
use uefi_wrapper as uefi;

#[no_mangle]
pub extern "win64" fn efi_main(_: uefi::Handle, mut st: uefi::SystemTable) -> ! {
    let mut stdout = st.con_out();

    let s = stdout.reset_without_extension();
    s.expect("Failed to reset the console.");
    loop {
        writeln!(stdout, "hello world").unwrap();
    }
}

#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
