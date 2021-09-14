use {
    crate::process::{
        self,
        ipc::{receive, send, ReceiveFrom},
    },
    core::{convert::TryInto, mem::MaybeUninit, ptr},
    ipc_api::message::Message,
    num_traits::FromPrimitive,
    os_units::Bytes,
    pid::Pid,
    x86_64::VirtAddr,
};

pub(crate) fn main() -> ! {
    loop {
        loop_iteration();
    }
}

fn loop_iteration() {
    let message = receive_message();

    match FromPrimitive::from_u64(message.body.0) {
        Some(syscalls::Ty::Noop) => reply_ack(message.header.sender_pid),
        Some(syscalls::Ty::CopyDataFrom) => handle_copy_data_from(&message),
        None => log::warn!("Unrecognized message: {:?}", message),
    }
}

fn handle_copy_data_from(message: &Message) {
    let src_pid = Pid::new(message.body.1.try_into().unwrap());
    let src_addr = VirtAddr::new(message.body.2);
    let dst_addr = VirtAddr::new(message.body.3);
    let bytes = Bytes::new(message.body.4.try_into().unwrap());

    // TODO: Remove this limitation.
    assert!(bytes.as_usize() < 128, "`bytes` must be less than 128.");

    let data = process::enter_address_space_and_do(src_pid, || {
        let mut buffer = [0_u8; 128];

        unsafe {
            ptr::copy(src_addr.as_ptr(), buffer.as_mut_ptr(), bytes.as_usize());
        }

        buffer
    });

    unsafe {
        ptr::copy(data.as_ptr(), dst_addr.as_mut_ptr(), bytes.as_usize());
    }

    reply_ack(message.header.sender_pid);
}

fn reply_ack(to: Pid) {
    let r = send(to, Message::default());
    r.unwrap_or_else(|_| log::warn!("Failed to send a message to {}", to));
}

fn receive_message() -> Message {
    let mut m = MaybeUninit::uninit();

    receive(ReceiveFrom::Any, m.as_mut_ptr()).expect("Failed to receive a message.");

    // SAFETY: `receive` receives a message.
    unsafe { m.assume_init() }
}
