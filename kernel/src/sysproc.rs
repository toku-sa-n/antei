use {
    crate::process::{
        ipc::{receive, send, Message, ReceiveFrom},
        Pid,
    },
    core::mem::MaybeUninit,
};

pub(crate) fn main() -> ! {
    loop {
        log::info!("The sysproc is receiving a message.");
        let message = receive_message();

        log::info!("The sysproc received a message: {:#X?}", message);

        if message.body == 0x334 {
            log::info!("The sysproc is sending a message.");
            send(message.from, Message::new(Pid::default(), 0x0114_0514));
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
