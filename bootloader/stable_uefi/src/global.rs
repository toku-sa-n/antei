use super::Handle;
use super::SystemTable;
use conquer_once::spin::Lazy;
use spinning_top::Spinlock;

static HANDLE: Lazy<Spinlock<Option<Handle>>> = Lazy::new(|| Spinlock::new(None));
static SYSTEM_TABLE: Lazy<Spinlock<Option<SystemTable>>> = Lazy::new(|| Spinlock::new(None));

pub(super) fn init(h: Handle, st: SystemTable) {
    init_handle(h);
    init_system_table(st);
}

fn init_handle(h: Handle) {
    let gh = HANDLE.try_lock();
    let mut gh = gh.expect("Failed to lock the global EFI Handler.");

    if gh.is_none() {
        *gh = Some(h);
    } else {
        panic!("The global EFI Handler is already initialized.");
    }
}

fn init_system_table(st: SystemTable) {
    let gst = SYSTEM_TABLE.try_lock();
    let mut gst = gst.expect("Failed to lock the global System Table.");

    if gst.is_none() {
        *gst = Some(st);
    } else {
        panic!("The global System Table is already initialized.");
    }
}
