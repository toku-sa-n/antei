use {
    crate::process::ipc::{receive, send, ReceiveFrom},
    core::mem::MaybeUninit,
    ipc_api::message::{Body, Header, Message},
    pid::predefined,
};

pub(crate) fn main() -> ! {
    ipc();

    qemu::exit_success();
}

fn ipc() {
    let m = Message {
        header: Header::default(),
        body: Body(0x334, 0, 0, 0, 0),
    };
    send(predefined::SYSPROC, m).unwrap();

    let mut m = MaybeUninit::uninit();
    receive(ReceiveFrom::Pid(predefined::SYSPROC), m.as_mut_ptr()).unwrap();

    // SAFETY: `receive` receives a message.
    let m = unsafe { m.assume_init() };

    assert_eq!(m.body, Body(0x0114_0514, 0, 0, 0, 0), "Wrong message body.");
}
