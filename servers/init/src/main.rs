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
        syscalls::noop();
    }
}
