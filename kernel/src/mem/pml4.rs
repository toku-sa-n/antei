use {
    aligned_ptr::ptr,
    conquer_once::spin::OnceCell,
    spinning_top::Spinlock,
    x86_64::{structures::paging::RecursivePageTable, VirtAddr},
};

const RECURSIVE_ADDR: VirtAddr = VirtAddr::new_truncate(0xff7f_bfdf_e000);

static PML4: OnceCell<Spinlock<RecursivePageTable>> = OnceCell::uninit();

/// # Safety
///
/// - The recursive address `0xff7f_bfdf_e000` must point to the current working PML4.
/// - There must not exist any references that point to the current working PML4.
pub unsafe fn init() {
    // SAFETY: The caller must ensure that the recursive paging address must point to the current
    // working PML4.
    let working_pml4 = unsafe { ptr::as_mut(RECURSIVE_ADDR.as_mut_ptr()) };
    let working_pml4 = RecursivePageTable::new(working_pml4);
    let working_pml4 =
        working_pml4.expect("Failed to get a reference to the current working PML4.");

    let r = PML4.try_init_once(|| Spinlock::new(working_pml4));
    r.expect("Failed to initialize a reference to PML4.");
}
