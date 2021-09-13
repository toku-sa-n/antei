use {
    crate::process::ipc::{receive, send, ReceiveFrom},
    core::mem::MaybeUninit,
    ipc_api::message::{Body, Header, Message},
};

pub(crate) fn main() -> ! {
    loop {
        log::info!("The sysproc is receiving a message.");
        let message = receive_message();

        log::info!("The sysproc received a message: {:#X?}", message);

        if let Body(0x334, 0, 0, 0, 0) = message.body {
            log::info!("The sysproc is sending a message.");

            let reply = Message {
                header: Header::default(),
                body: Body(0x0114_0514, 0, 0, 0, 0),
            };

            send(message.header.sender_pid, reply).expect("Failed to send a message.");

            log::info!("The sysproc sent a message.");
        }
    }
}

fn receive_message() -> Message {
    let mut m = MaybeUninit::uninit();

    receive(ReceiveFrom::Any, m.as_mut_ptr()).expect("Failed to receive a message.");

    // SAFETY: `receive` receives a message.
    unsafe { m.assume_init() }
}
