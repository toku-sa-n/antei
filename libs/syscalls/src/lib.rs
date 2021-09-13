#![no_std]

use {
    ipc::{
        message::{Body, Header, Message},
        ReceiveFrom,
    },
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

pub enum Ty {
    Noop,
}
