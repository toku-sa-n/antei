#![no_std]
#![no_main]

extern crate vm_server as _;

use ipc::ReceiveFrom;

#[no_mangle]
fn main() -> ! {
    loop {
        let _ = ipc::receive(ReceiveFrom::Any);
    }
}
