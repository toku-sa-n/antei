#![no_std]
#![no_main]

extern crate qemu_serial as _;

use {
    ipc::Message,
    spinning_top::{const_spinlock, Spinlock, SpinlockGuard},
    uart_16550::SerialPort,
};

static PORT: Spinlock<SerialPort> = const_spinlock(unsafe { SerialPort::new(0x3f8) });

#[no_mangle]
fn main() -> ! {
    driver::run(Handler);
}

fn lock<'a>() -> SpinlockGuard<'a, SerialPort> {
    PORT.lock()
}

struct Handler;
impl driver::Handler for Handler {
    fn init(&mut self) {
        lock().init();
    }

    fn open(&mut self, message: &Message) -> driver::Status {
        0
    }

    fn close(&mut self, message: &Message) -> driver::Status {
        0
    }

    fn read(&mut self, message: &Message) -> driver::Status {
        todo!()
    }

    fn write(&mut self, message: &Message) -> driver::Status {
        todo!()
    }

    fn ioctl(&mut self, message: &Message) -> driver::Status {
        0
    }
}
