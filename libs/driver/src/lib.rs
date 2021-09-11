#![no_std]

use ipc::Message;

pub trait Handler {
    fn init(&mut self);
    fn open(&mut self, message: &Message);
    fn close(&mut self, message: &Message);
    fn read(&mut self, message: &Message);
    fn write(&mut self, message: &Message);
    fn ioctl(&mut self, message: &Message);
}

pub fn run(mut handler: impl Handler) -> ! {
    handler.init();

    main_loop(handler);
}

fn main_loop(mut handler: impl Handler) -> ! {
    loop {
        loop_iteration(&mut handler);
    }
}

fn loop_iteration(_handler: &mut impl Handler) {}
