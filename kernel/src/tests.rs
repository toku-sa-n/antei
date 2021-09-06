use {
    crate::process::{
        ipc::{receive, send, ReceiveFrom},
        Pid,
    },
    core::mem::MaybeUninit,
    message::{Body, Header, Message},
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
    send(Pid::new(2), m);

    let mut m = MaybeUninit::uninit();
    receive(ReceiveFrom::Pid(Pid::new(2)), m.as_mut_ptr());

    // SAFETY: `receive` receives a message.
    let m = unsafe { m.assume_init() };

    assert_eq!(m.body, Body(0x0114_0514, 0, 0, 0, 0), "Wrong message body.");
}
