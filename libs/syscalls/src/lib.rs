#![no_std]

use {
    core::convert::TryInto,
    ipc::message::{Body, Header, Message},
    num_derive::FromPrimitive,
    num_traits::FromPrimitive,
    os_units::Bytes,
    pid::{predefined, Pid},
    x86_64::{PhysAddr, VirtAddr},
};

/// # Panics
///
/// This function panics if the kernel did not reply an empty message.
pub fn noop() {
    let message = Message {
        header: Header::default(),
        body: Body(Ty::Noop as _, 0, 0, 0, 0),
    };

    ipc::send(predefined::SYSPROC, message);

    let reply = ipc::receive(predefined::SYSPROC.into());

    assert_eq!(reply.body, Body::default());
}

/// # Safety
///
/// The all arguments must be correct.
///
/// # Panics
///
/// This function panics if one of the following conditions is satisfied.
/// - The kernel did not reply an empty message.
/// - `bytes < 128`. This is the current implementation limitation.
pub unsafe fn copy_data_from(src_pid: Pid, src_addr: VirtAddr, dst_addr: VirtAddr, bytes: Bytes) {
    // TODO: Remove this limitation.
    assert!(bytes.as_usize() < 128, "`bytes` must be less than 128.");

    let message = Message {
        header: Header::default(),
        body: Body(
            Ty::CopyDataFrom as _,
            src_pid.as_usize().try_into().unwrap(),
            src_addr.as_u64(),
            dst_addr.as_u64(),
            bytes.as_usize().try_into().unwrap(),
        ),
    };

    ipc::send(predefined::SYSPROC, message);

    let reply = ipc::receive(predefined::SYSPROC.into());

    assert_eq!(reply.body, Body::default());
}

/// # Panics
///
/// This function panics if the kernel sent an invalid bits order.
#[must_use]
pub fn get_screen_info() -> ScreenInfo {
    let message = Message {
        header: Header::default(),
        body: Body(Ty::GetScreenInfo as _, 0, 0, 0, 0),
    };

    ipc::send(predefined::SYSPROC, message);

    let reply = ipc::receive(predefined::SYSPROC.into());

    ScreenInfo {
        resolution_x: reply.body.0.try_into().unwrap(),
        resolution_y: reply.body.1.try_into().unwrap(),
        bits_order: FromPrimitive::from_u64(reply.body.2).expect("Invalid bits order."),
        scan_line_width: reply.body.3.try_into().unwrap(),
        frame_buffer: PhysAddr::new(reply.body.4),
    }
}

/// # Safety
///
/// The caller must ensure that the memory region is the correct one.
///
/// # Panics
///
/// This function panics if the kernel failed to map the memory.
#[must_use]
pub unsafe fn map_memory(start: PhysAddr, len: Bytes) -> VirtAddr {
    let message = Message {
        header: Header::default(),
        body: Body(
            Ty::MapMemory as _,
            start.as_u64(),
            len.as_usize().try_into().unwrap(),
            0,
            0,
        ),
    };

    ipc::send(predefined::SYSPROC, message);

    let reply = ipc::receive(predefined::SYSPROC.into());

    assert_ne!(reply.body.0, 0, "Failed to map memory.");

    VirtAddr::new(reply.body.0)
}

/// # Panics
///
/// This function panics if `s.len() >= 128`.
pub fn write(s: &str) {
    assert!(
        s.len() < 128,
        "The current limitation is that the length must be less than 128 characters."
    );

    let message = Message {
        header: Header::default(),
        body: Body(
            Ty::Write as _,
            s.as_ptr() as _,
            s.len().try_into().unwrap(),
            0,
            0,
        ),
    };

    ipc::send(predefined::TTY, message);

    let reply = ipc::receive(predefined::TTY.into());

    assert_eq!(
        reply.body,
        Body::default(),
        "The tty sent an invalid message."
    );
}

#[must_use]
pub fn pm_syncs_with_kernel() -> Option<Message> {
    const NOT_END: u64 = 1;

    let message = Message {
        header: Header::default(),
        body: Body(Ty::PmSyncsWithKernel as _, 0, 0, 0, 0),
    };

    ipc::send(predefined::SYSPROC, message);

    let reply = ipc::receive(predefined::SYSPROC.into());

    (reply.body.0 == NOT_END).then(|| reply)
}

#[must_use]
pub fn inl(port: u16) -> u32 {
    let message = Message {
        header: Header::default(),
        body: Body(Ty::Inl as _, port.into(), 0, 0, 0),
    };

    ipc::send(predefined::SYSPROC, message);

    let reply = ipc::receive(predefined::SYSPROC.into());

    reply.body.0.try_into().unwrap()
}

/// # Panics
///
/// This function panics if the kernel did not reply an empty message.
pub fn outl(port: u16, value: u32) {
    let message = Message {
        header: Header::default(),
        body: Body(Ty::Outl as _, port.into(), value.into(), 0, 0),
    };

    ipc::send(predefined::SYSPROC, message);

    let reply = ipc::receive(predefined::SYSPROC.into());

    assert_eq!(reply, Message::default());
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScreenInfo {
    resolution_x: u32,
    resolution_y: u32,
    bits_order: BitsOrder,
    scan_line_width: u32,
    frame_buffer: PhysAddr,
}
impl ScreenInfo {
    #[must_use]
    pub fn resolution_x(&self) -> u32 {
        self.resolution_x
    }

    #[must_use]
    pub fn resolution_y(&self) -> u32 {
        self.resolution_y
    }

    #[must_use]
    pub fn bits_order(&self) -> BitsOrder {
        self.bits_order
    }

    #[must_use]
    pub fn scan_line_width(&self) -> u32 {
        self.scan_line_width
    }

    #[must_use]
    pub fn frame_buffer(&self) -> PhysAddr {
        self.frame_buffer
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BitsOrder {
    RedGreenBlueReserved,
    BlueGreenRedReserved,
}

#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ty {
    Noop,
    CopyDataFrom,
    GetScreenInfo,
    MapMemory,
    Write,
    PmSyncsWithKernel,
    Inl,
    Outl,
}
