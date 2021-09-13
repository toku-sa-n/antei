#![no_std]

use {
    ipc::{
        message::{Body, Header},
        Message, ReceiveFrom,
    },
    num_derive::FromPrimitive,
    num_traits::FromPrimitive,
};

pub type Status = i32;

pub trait Handler {
    fn init(&mut self);
    fn open(&mut self, message: &Message) -> Status;
    fn close(&mut self, message: &Message) -> Status;
    fn read(&mut self, message: &Message) -> Status;
    fn write(&mut self, message: &Message) -> Status;
    fn ioctl(&mut self, message: &Message) -> Status;
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

fn loop_iteration(handler: &mut impl Handler) {
    let message = ipc::receive(ReceiveFrom::Any);

    let status = match FromPrimitive::from_u64(message.body.0) {
        Some(OperationType::Open) => handler.open(&message),
        Some(OperationType::Close) => handler.close(&message),
        Some(OperationType::Read) => handler.read(&message),
        Some(OperationType::Write) => handler.write(&message),
        Some(OperationType::Ioctl) => handler.ioctl(&message),
        None => -1,
    };

    #[allow(clippy::cast_sign_loss)]
    let reply = Message {
        header: Header::default(),
        body: Body(status as u64, 0, 0, 0, 0),
    };

    ipc::send(message.header.sender_pid, reply);
}

#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum OperationType {
    Open,
    Close,
    Read,
    Write,
    Ioctl,
}
