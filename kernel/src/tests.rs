use {
    crate::process::{
        ipc::{receive, send, Message, ReceiveFrom},
        Pid,
    },
    core::mem::MaybeUninit,
};

pub(crate) fn main() -> ! {
    ipc();

    qemu::exit_success();
}

fn ipc() {
    send(Pid::new(2), Message::new(Pid::default(), 0x334));

    let mut m = MaybeUninit::uninit();
    receive(ReceiveFrom::Pid(Pid::new(2)), m.as_mut_ptr());

    // SAFETY: `receive` receives a message.
    let m = unsafe { m.assume_init() };

    assert_eq!(m.body, 0x0114_0514, "Wrong message body.");
}
