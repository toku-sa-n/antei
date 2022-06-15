use {
    crate::process::ipc::{receive, send, ReceiveFrom},
    core::{convert::TryInto, mem::MaybeUninit},
    ipc_api::message::{Body, Header, Message},
    pid::predefined,
    x86_64::{instructions::hlt, VirtAddr},
};

static DATA: &str = "Take the initiative and shoot flame. That's all.";

pub(crate) fn main_1() -> ! {
    ipc();

    let mut m = MaybeUninit::uninit();
    receive(predefined::TEST_2.into(), m.as_mut_ptr()).unwrap();

    let m = unsafe { m.assume_init() };

    let mut buffer = [0; 128];

    let src_addr = VirtAddr::new(m.body.0);
    let dst_addr = VirtAddr::from_ptr(buffer.as_mut_ptr());
    let count = DATA.len();

    let message = Message {
        header: Header::default(),
        body: Body(
            syscalls::Ty::CopyDataFrom as _,
            predefined::TEST_2.as_usize().try_into().unwrap(),
            src_addr.as_u64(),
            dst_addr.as_u64(),
            count.try_into().unwrap(),
        ),
    };

    send(predefined::SYSPROC, message).unwrap();

    let mut m = MaybeUninit::uninit();
    receive(predefined::SYSPROC.into(), m.as_mut_ptr()).unwrap();

    assert_eq!(unsafe { m.assume_init().body }, Body::default());

    assert_eq!(&buffer[..count], DATA.as_bytes());

    let mut m = MaybeUninit::uninit();
    receive(predefined::TEST_USER_APP.into(), m.as_mut_ptr()).unwrap();

    assert_eq!(unsafe { m.assume_init().body }, Body::default());

    qemu::exit_success();
}

pub(crate) fn main_2() -> ! {
    let m = Message {
        header: Header::default(),
        body: Body(DATA.as_ptr() as _, DATA.len().try_into().unwrap(), 0, 0, 0),
    };
    send(predefined::TEST_1, m).unwrap();

    loop {
        hlt();
    }
}

fn ipc() {
    let m = Message {
        header: Header::default(),
        body: Body(syscalls::Ty::Noop as _, 0, 0, 0, 0),
    };
    send(predefined::SYSPROC, m).unwrap();

    let mut m = MaybeUninit::uninit();
    receive(ReceiveFrom::Pid(predefined::SYSPROC), m.as_mut_ptr()).unwrap();

    // SAFETY: `receive` receives a message.
    let m = unsafe { m.assume_init() };

    assert_eq!(m.body, Body::default());
}
