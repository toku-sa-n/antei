use {
    aligned_ptr::ptr,
    x86_64::{
        registers::control::{Cr0, Cr0Flags, Cr3, Cr4, Cr4Flags, Efer, EferFlags},
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
pub unsafe fn init() {
    // SAFETY: The caller must uphold that the physical and virtual addresses of the PML4 is the
    // same value.
    unsafe {
        enable_recursive_paging();
    }

    enable_global_flag();
    enable_no_execute_flag();
}

pub(crate) fn pml4_addr() -> PhysAddr {
    let f = Cr3::read().0;
    f.start_address()
}

/// # Safety
///
/// This function assumes that the physical and virtual addresses of the PML4 is the same value.
#[allow(clippy::module_name_repetitions)]
unsafe fn enable_recursive_paging() {
    // SAFETY: The caller must uphold that the physical and virtual addresses of the PML4 is the
    // same value.
    edit_page_tables(|| unsafe {
        set_recursive_entry();
    });
}

fn enable_global_flag() {
    // SAFETY: This operation does not violate memory safety.
    unsafe {
        Cr4::update(|cr4| cr4.insert(Cr4Flags::PAGE_GLOBAL));
    }
}

fn enable_no_execute_flag() {
    // SAFETY: This operation does not violate memory safety.
    unsafe {
        Efer::update(|efer| efer.insert(EferFlags::NO_EXECUTE_ENABLE));
    }
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

    let flags = PageTableFlags::PRESENT
        | PageTableFlags::WRITABLE
        | PageTableFlags::GLOBAL
        | PageTableFlags::NO_EXECUTE;

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
