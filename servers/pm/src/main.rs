#![no_std]
#![no_main]

extern crate pm as _;

use ipc::ReceiveFrom;

#[no_mangle]
fn main() -> ! {
    loop {
        let _ = ipc::receive(ReceiveFrom::Any);
    }
}
