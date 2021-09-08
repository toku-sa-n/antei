use {
    crate::process::ipc::{receive, send, ReceiveFrom},
    core::mem::MaybeUninit,
    ipc_api::{Body, Header, Message},
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

            send(message.header.sender_pid.into(), reply);
            log::info!("The sysproc sent a message.");
        }
    }
}

fn receive_message() -> Message {
    let mut m = MaybeUninit::uninit();

    receive(ReceiveFrom::Any, m.as_mut_ptr());

    // SAFETY: `receive` receives a message.
    unsafe { m.assume_init() }
}
