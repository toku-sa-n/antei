use {
    spinning_top::{const_spinlock, Spinlock, SpinlockGuard},
    x86_64::{structures::tss::TaskStateSegment, VirtAddr},
};

static TSS: Spinlock<TaskStateSegment> = const_spinlock(TaskStateSegment::new());

pub(super) fn set_kernel_stack_addr(a: VirtAddr) {
    tss().privilege_stack_table[0] = a;
}

/// # Safety
///
/// TSS must not be modified while the returned reference is alive.
pub(super) unsafe fn as_ref() -> &'static TaskStateSegment {
    // SAFETY: The caller must ensure that TSS is not modified while this reference is alive.
    unsafe { &*TSS.data_ptr() }
}

fn tss<'a>() -> SpinlockGuard<'a, TaskStateSegment> {
    let t = TSS.try_lock();

    t.expect("Failed to lock TSS.")
}
