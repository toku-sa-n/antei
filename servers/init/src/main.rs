#![no_std]
#![no_main]

extern crate init as _;

use {
    ipc::{
        message::{Body, Header, Message},
        ReceiveFrom,
    },
    pid::{predefined, Pid},
};

#[no_mangle]
fn main() -> ! {
    loop {
        let message = Message {
            header: Header::default(),
            body: Body(0x334, 0, 0, 0, 0),
        };

        ipc::send(predefined::SYSPROC, message);

        let message = ipc::receive(ReceiveFrom::Pid(predefined::SYSPROC));

        assert_eq!(message.body, Body(0x0114_0514, 0, 0, 0, 0));

        assert!(ipc::try_receive(ReceiveFrom::Pid(Pid::new(6))).is_err());
    }
}
