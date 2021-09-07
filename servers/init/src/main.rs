#![no_std]
#![no_main]

use {
    ipc::{receive, send},
    message::{Body, Header, Message},
};

extern crate init as _;

#[no_mangle]
fn main() -> ! {
    loop {
        let m = Message {
            header: Header::default(),
            body: Body(0x334, 0, 0, 0, 0),
        };

        send(2, m);

        let m = receive(2);
        assert_eq!(m.body, Body(0x0114_0514, 0, 0, 0, 0));
    }
}
