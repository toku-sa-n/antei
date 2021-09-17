use {
    crate::{
        boot_info,
        process::{
            self,
            ipc::{receive, send, ReceiveFrom},
        },
    },
    core::{
        convert::TryInto,
        mem::MaybeUninit,
        ptr,
        sync::atomic::{AtomicUsize, Ordering},
    },
    ipc_api::message::{Body, Header, Message},
    num_traits::FromPrimitive,
    os_units::Bytes,
    pid::Pid,
    uefi::protocols::console::graphics_output::{
        PIXEL_BLUE_GREEN_RED_RESERVED_8_BIT_PER_COLOR,
        PIXEL_RED_GREEN_BLUE_RESERVED_8_BIT_PER_COLOR,
    },
    x86_64::{
        instructions::port::{PortReadOnly, PortWriteOnly},
        structures::paging::PageTableFlags,
        PhysAddr, VirtAddr,
    },
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
        Some(syscalls::Ty::GetScreenInfo) => handle_get_screen_info(message.header.sender_pid),
        Some(syscalls::Ty::MapMemory) => handle_map_memory(&message),
        Some(syscalls::Ty::PmSyncsWithKernel) => handle_pm_syncs_with_kernel(&message),
        Some(syscalls::Ty::Inl) => handle_inl(&message),
        Some(syscalls::Ty::Outl) => handle_outl(&message),
        _ => log::warn!("Unrecognized message: {:?}", message),
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

    process::enter_address_space_and_do(message.header.sender_pid, || unsafe {
        ptr::copy(data.as_ptr(), dst_addr.as_mut_ptr(), bytes.as_usize());
    });

    reply_ack(message.header.sender_pid);
}

fn handle_get_screen_info(to: Pid) {
    let boot_info = boot_info::get();
    let gop_info = boot_info.gop_mode_information();

    let resolution_x = gop_info.horizontal_resolution;
    let resolution_y = gop_info.vertical_resolution;
    let bits_order = match gop_info.pixel_format {
        PIXEL_RED_GREEN_BLUE_RESERVED_8_BIT_PER_COLOR => 0,
        PIXEL_BLUE_GREEN_RED_RESERVED_8_BIT_PER_COLOR => 1,
        _ => todo!(),
    };
    let scan_line = gop_info.pixels_per_scan_line;
    let frame_buffer = boot_info.frame_buffer();

    let message = Message {
        header: Header::default(),
        body: Body(
            resolution_x.into(),
            resolution_y.into(),
            bits_order,
            scan_line.into(),
            frame_buffer.as_u64(),
        ),
    };

    let r = send(to, message);
    r.unwrap_or_else(|_| log::warn!("Failed to send a message to {}", to));
}

fn handle_map_memory(message: &Message) {
    let to = message.header.sender_pid;

    let start = PhysAddr::new(message.body.1);
    let len = Bytes::new(message.body.2.try_into().unwrap());
    let flags =
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;

    let virt =
        process::enter_address_space_and_do(to, || unsafe { vm::map_user(start, len, flags) });

    let reply = Message {
        header: Header::default(),
        body: Body(virt.as_u64(), 0, 0, 0, 0),
    };

    let r = send(to, reply);
    r.unwrap_or_else(|_| log::warn!("Failed to send a message to {}", to));
}

fn handle_pm_syncs_with_kernel(message: &Message) {
    static NEXT_PID: AtomicUsize = AtomicUsize::new(0);

    const PM_PROC_INFO: u64 = 1;
    const PM_PROC_END: u64 = 2;

    assert_eq!(message.header.sender_pid, pid::predefined::PM);

    let pid = Pid::new(NEXT_PID.fetch_add(1, Ordering::Relaxed));

    let pm_msg = Message {
        header: Header::default(),
        body: if process::process_exists(pid) {
            Body(PM_PROC_INFO, pid.as_usize().try_into().unwrap(), 0, 0, 0)
        } else {
            Body(PM_PROC_END, 0, 0, 0, 0)
        },
    };

    let r = send(message.header.sender_pid, pm_msg);
    r.expect("Failed to sync with PM.");
}

fn handle_inl(message: &Message) {
    let port = message.body.1;

    let ret = if let Ok(port) = port.try_into() {
        let mut port = PortReadOnly::<u32>::new(port);

        unsafe { port.read() }
    } else {
        u32::MAX
    };

    let reply = Message {
        header: Header::default(),
        body: Body(ret.into(), 0, 0, 0, 0),
    };

    let r = send(message.header.sender_pid, reply);
    r.unwrap_or_else(|_| log::warn!("Failed to send a reply."));
}

fn handle_outl(message: &Message) {
    let port = message.body.1;
    let value = message.body.2;

    if let (Ok(port), Ok(value)) = (port.try_into(), value.try_into()) {
        let mut port = PortWriteOnly::<u32>::new(port);

        unsafe { port.write(value) }
    }

    let r = send(message.header.sender_pid, Message::default());
    r.unwrap_or_else(|_| log::warn!("Failed to send a reply."));
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
