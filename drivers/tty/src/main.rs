#![no_std]
#![no_main]

extern crate tty as _;

use {core::convert::TryInto, os_units::Bytes};

#[no_mangle]
fn main() -> ! {
    let screen_info = syscalls::get_screen_info();
    let len = screen_info.scan_line_width() * screen_info.resolution_y() * 4;
    let len = Bytes::new(len.try_into().unwrap());

    let virt = syscalls::map_memory(screen_info.frame_buffer(), len);

    unsafe {
        core::ptr::write_bytes(virt.as_mut_ptr::<u8>(), 255, len.as_usize());
    }

    loop {
        core::hint::spin_loop();
    }
}
