#![no_std]

use core::panic::PanicInfo;
use r_efi::efi;

#[no_mangle]
pub extern "win64" fn efi_main(_: efi::Handle, st: efi::SystemTable) -> ! {
    let stdout = unsafe { &mut *(st.con_out) };
    let string = "hello world".as_bytes();
    let mut buf = [0u16; 32];

    for i in 0..string.len() {
        buf[i] = string[i] as u16;
    }

    (stdout.reset)(stdout, false.into());
    loop {
        (stdout.output_string)(stdout, buf.as_mut_ptr());
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
