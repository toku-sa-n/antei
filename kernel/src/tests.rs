use {
    crate::process::{
        ipc::{receive, send, Message, ReceiveFrom},
        Pid,
    },
    core::mem::MaybeUninit,
};

pub(crate) fn main() -> ! {
    loop {
        log::info!("The test process is sending a message.");
        send(Pid::new(2), Message::new(Pid::default(), 0x334));
        log::info!("The test process sent a message.");

        log::info!("The test process is receiving a message.");
        let mut m = MaybeUninit::uninit();
        receive(ReceiveFrom::Pid(Pid::new(2)), m.as_mut_ptr());

        unsafe {
            log::info!("The test process received a message: {:?}", m.assume_init());
        }
    }
}
