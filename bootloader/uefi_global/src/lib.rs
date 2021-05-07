#![no_std]

mod io;

use conquer_once::spin::Lazy;
use spinning_top::Spinlock;

static HANDLE_WRAPPER: Lazy<Spinlock<Option<HandleWrapper>>> = Lazy::new(|| Spinlock::new(None));
static SYSTEM_TABLE_WRAPPER: Lazy<Spinlock<Option<SystemTableWrapper>>> =
    Lazy::new(|| Spinlock::new(None));

struct HandleWrapper(uefi_wrapper::Handle);
// SAFETY: UEFI applications have only one thread.
unsafe impl Send for HandleWrapper {}

struct SystemTableWrapper(uefi_wrapper::SystemTable);
// SAFETY: UEFI applications have only one thread
unsafe impl Send for SystemTableWrapper {}

pub fn init(h: uefi_wrapper::Handle, st: uefi_wrapper::SystemTable) {
    init_handle(h);
    init_system_table(st);
}

fn init_handle(h: uefi_wrapper::Handle) {
    let gh = HANDLE_WRAPPER.try_lock();
    let mut gh = gh.expect("Failed to lock the global EFI Handler.");

    if gh.is_some() {
        panic!("The global handler is already initialized.");
    } else {
        *gh = Some(HandleWrapper(h));
    }
}

fn init_system_table(st: uefi_wrapper::SystemTable) {
    let gst = SYSTEM_TABLE_WRAPPER.try_lock();
    let mut gst = gst.expect("Failed to lock the global System Table.");

    if gst.is_some() {
        panic!("The global System Table is already initialized.");
    } else {
        *gst = Some(SystemTableWrapper(st));
    }
}
