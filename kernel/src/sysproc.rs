use {
    crate::process::ipc::{receive, send, ReceiveFrom},
    core::{convert::TryInto, mem::MaybeUninit},
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

            match message.header.sender_pid.try_into() {
                Ok(pid) => {
                    send(pid, reply);

                    log::info!("The sysproc sent a message.");
                }
                Err(e) => {
                    log::error!("The pid indicated an error: {:?}", e);
                }
            }
        }
    }
}

fn receive_message() -> Message {
    let mut m = MaybeUninit::uninit();

    receive(ReceiveFrom::Any, m.as_mut_ptr());

    // SAFETY: `receive` receives a message.
    unsafe { m.assume_init() }
}
