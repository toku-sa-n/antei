#![no_std]

extern crate rlibc as _;

mod font;
mod vram;
mod writer;

#[doc(hidden)]
pub use writer::_print;
use {
    core::{convert::TryInto, str},
    ipc::{Message, ReceiveFrom},
    num_traits::FromPrimitive,
    os_units::Bytes,
    x86_64::VirtAddr,
};

pub fn init() {
    let screen_info = syscalls::get_screen_info();

    vram::init(screen_info);
}

pub fn main_loop() -> ! {
    loop {
        loop_iteration();
    }
}

fn loop_iteration() {
    let message = ipc::receive(ReceiveFrom::Any);

    if let Some(syscalls::Ty::Write) = FromPrimitive::from_u64(message.body.0) {
        handle_write(&message);
    } else {
        println!("Unrecognized message: {:?}", message);
    }
}

fn handle_write(message: &Message) {
    let src = VirtAddr::new(message.body.1);
    let len = Bytes::new(message.body.2.try_into().unwrap());

    let mut buffer = [0_u8; 128];

    let dst_addr = VirtAddr::from_ptr(buffer.as_mut_ptr());

    unsafe {
        syscalls::copy_data_from(message.header.sender_pid, src, dst_addr, len);
    }

    if let Ok(s) = str::from_utf8(&buffer[..len.as_usize()]) {
        print!("{}", s);
    } else {
        println!("Received non-UTF-8 string.");
    }

    ipc::send(message.header.sender_pid, Message::default());
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
