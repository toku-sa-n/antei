#![no_std]

pub mod io;

use conquer_once::spin::Lazy;
use core::panic::PanicInfo;
use log::error;
use spinning_top::lock_api::MappedMutexGuard;
use spinning_top::RawSpinlock;
use spinning_top::Spinlock;
use spinning_top::SpinlockGuard;

static HANDLE_WRAPPER: Lazy<Spinlock<Option<Handle>>> = Lazy::new(|| Spinlock::new(None));
static SYSTEM_TABLE_WRAPPER: Lazy<Spinlock<Option<SystemTable>>> =
    Lazy::new(|| Spinlock::new(None));

#[repr(transparent)]
#[derive(Debug)]
pub struct Handle(uefi_wrapper::Handle);
// SAFETY: UEFI applications have only one thread.
unsafe impl Send for Handle {}

#[repr(transparent)]
#[derive(Debug)]
pub struct SystemTable(uefi_wrapper::SystemTable);
// SAFETY: UEFI applications have only one thread
unsafe impl Send for SystemTable {}

pub fn init(h: Handle, st: SystemTable) {
    init_handle(h);
    init_system_table(st);
    io::init();
}

pub(crate) fn system_table<'a>() -> MappedMutexGuard<'a, RawSpinlock, uefi_wrapper::SystemTable> {
    let st = SYSTEM_TABLE_WRAPPER.try_lock();
    let st = st.expect("Failed to lock the global System Table.");

    SpinlockGuard::map(st, |st| {
        let st = st.as_mut();
        let st = st.expect("The global System Table is not initialized.");
        &mut st.0
    })
}

fn init_handle(h: Handle) {
    let gh = HANDLE_WRAPPER.try_lock();
    let mut gh = gh.expect("Failed to lock the global EFI Handler.");

    assert!(
        gh.is_none(),
        "The global EFI Handler is already initialized."
    );

    *gh = Some(h);
}

fn init_system_table(st: SystemTable) {
    let gst = SYSTEM_TABLE_WRAPPER.try_lock();
    let mut gst = gst.expect("Failed to lock the global System Table.");

    assert!(
        gst.is_none(),
        "The global System Table is already initialized."
    );

    *gst = Some(st);
}

#[panic_handler]
fn panic(i: &PanicInfo<'_>) -> ! {
    error!("{}", i);

    loop {
        x86_64::instructions::hlt();
    }
}
