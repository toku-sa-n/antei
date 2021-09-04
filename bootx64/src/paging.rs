use {
    aligned_ptr::ptr,
    x86_64::{
        registers::control::{Cr0, Cr0Flags, Cr3},
        structures::paging::{PageTable, PageTableFlags},
        PhysAddr, VirtAddr,
    },
};

pub(crate) fn edit_page_tables<T>(f: impl FnOnce() -> T) -> T {
    disable_write_protect();
    let r = f();
    enable_write_protect();
    r
}

/// # Safety
///
/// This function assumes that the physical and virtual addresses of the PML4 is the same value.
#[allow(clippy::module_name_repetitions)]
pub unsafe fn enable_recursive_paging() {
    // SAFETY: The caller must uphold that the physical and virtual addresses of the PML4 is the
    // same value.
    edit_page_tables(|| unsafe {
        set_recursive_entry();
    });
}

pub(crate) fn pml4_addr() -> PhysAddr {
    let f = Cr3::read().0;
    f.start_address()
}

/// # Safety
///
/// This function assumes that the physical and virtual addresses of the PML4 is the same value.
unsafe fn set_recursive_entry() {
    let p = pml4_addr();
    let v = VirtAddr::new(p.as_u64());

    // SAFETY: The caller must ensure that the physical and virtual addresses of the PML4 is the
    // same value.
    let table: &mut PageTable = unsafe { ptr::as_mut(v.as_mut_ptr()) };

    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::GLOBAL;
    table[510].set_addr(p, flags);
}

fn enable_write_protect() {
    // SAFETY: Enabling the write protection does not affect the memory safety.
    unsafe {
        Cr0::update(|cr0| cr0.insert(Cr0Flags::WRITE_PROTECT));
    }
}

fn disable_write_protect() {
    // SAFETY: Disabling the write protection does not affect the memory safety.
    unsafe {
        Cr0::update(|cr0| cr0.remove(Cr0Flags::WRITE_PROTECT));
    }
}
