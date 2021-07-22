use {
    aligned_ptr::ptr,
    conquer_once::spin::OnceCell,
    spinning_top::{Spinlock, SpinlockGuard},
    x86_64::{structures::paging::RecursivePageTable, VirtAddr},
};

const RECURSIVE_ADDR: VirtAddr = VirtAddr::new_truncate(0xff7f_bfdf_e000);

static PML4: OnceCell<Spinlock<RecursivePageTable<'_>>> = OnceCell::uninit();

/// # Safety
///
/// - The recursive address `0xff7f_bfdf_e000` must point to the current working PML4.
/// - There must not exist any references that point to the current working PML4.
pub unsafe fn init() {
    init_static();
    unmap_all_user_regions();
}

fn init_static() {
    // SAFETY: The caller must ensure that the recursive paging address must point to the current
    // working PML4.
    let working_pml4 = unsafe { ptr::as_mut(RECURSIVE_ADDR.as_mut_ptr()) };
    let working_pml4 = RecursivePageTable::new(working_pml4);
    let working_pml4 =
        working_pml4.expect("Failed to get a reference to the current working PML4.");

    let r = PML4.try_init_once(|| Spinlock::new(working_pml4));
    r.expect("Failed to initialize a reference to PML4.");
}

fn unmap_all_user_regions() {
    let mut pml4 = pml4();
    let pml4 = pml4.level_4_table();

    for i in 0..510 {
        pml4[i].set_unused();
    }
}

fn pml4<'a>() -> SpinlockGuard<'a, RecursivePageTable<'static>> {
    let pml4 = PML4.try_get();
    let pml4 = pml4.expect("`pml4::init` is not called.");
    let pml4 = pml4.try_lock();

    pml4.expect("Failed to acquire kernel's PML4.")
}
