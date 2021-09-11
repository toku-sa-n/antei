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

        ipc::try_send(2, message).unwrap();

        let message = ipc::try_receive(ReceiveFrom::Pid(2)).unwrap();

        assert_eq!(message.body, Body(0x0114_0514, 0, 0, 0, 0));

        assert!(ipc::try_receive(ReceiveFrom::Pid(6)).is_err());
    }
}
