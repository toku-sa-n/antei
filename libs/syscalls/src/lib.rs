#![no_std]

use {
    ipc::{
        message::{Body, Header, Message},
        ReceiveFrom,
    },
    num_derive::FromPrimitive,
    pid::predefined,
};

pub fn noop() {
    let message = Message {
        header: Header::default(),
        body: Body(Ty::Noop as _, 0, 0, 0, 0),
    };

    ipc::send(predefined::SYSPROC, message);

    let reply = ipc::receive(ReceiveFrom::Pid(predefined::SYSPROC));

    assert_eq!(reply.body, Body::default());
}

#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ty {
    Noop,
}
