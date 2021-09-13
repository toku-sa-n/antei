use {
    crate::process::ipc::{receive, send, ReceiveFrom},
    core::mem::MaybeUninit,
    ipc_api::message::Message,
    num_traits::FromPrimitive,
};

pub(crate) fn main() -> ! {
    loop {
        loop_iteration();
    }
}

fn loop_iteration() {
    let message = receive_message();

    if let Some(syscalls::Ty::Noop) = FromPrimitive::from_u64(message.body.0) {
        let to = message.header.sender_pid;

        let r = send(to, Message::default());
        r.unwrap_or_else(|_| log::warn!("Failed to send a message to {}", to));
    } else {
        log::warn!("Unrecognized message: {:?}", message);
    }
}

fn receive_message() -> Message {
    let mut m = MaybeUninit::uninit();

    receive(ReceiveFrom::Any, m.as_mut_ptr()).expect("Failed to receive a message.");

    // SAFETY: `receive` receives a message.
    unsafe { m.assume_init() }
}
