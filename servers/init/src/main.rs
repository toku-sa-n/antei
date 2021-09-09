#![no_std]
#![no_main]

extern crate init as _;

use ipc::{
    message::{Body, Header, Message},
    ReceiveFrom,
};

#[no_mangle]
fn main() -> ! {
    loop {
        let message = Message {
            header: Header::default(),
            body: Body(0x334, 0, 0, 0, 0),
        };

        ipc::send(2, message).unwrap();

        let message = ipc::receive(ReceiveFrom::Pid(2)).unwrap();

        assert_eq!(message.body, Body(0x0114_0514, 0, 0, 0, 0));

        assert!(ipc::receive(ReceiveFrom::Pid(6)).is_err());
    }
}
