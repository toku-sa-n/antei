use aligned_ptr::ptr;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::PageTable;
use x86_64::structures::paging::PageTableFlags;
use x86_64::PhysAddr;
use x86_64::VirtAddr;

pub(crate) fn edit_page_tables(f: impl FnOnce()) {
    disable_write_protect();
    f();
    enable_write_protect();
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
    })
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

    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    table[510].set_addr(p, flags);
}

fn enable_write_protect() {
    extern "C" {
        fn asm_enable_page_table_write_protect();
    }

    // SAFETY: Enabling the write protection does not affect the memory safety.
    unsafe { asm_enable_page_table_write_protect() }
}

fn disable_write_protect() {
    extern "C" {
        fn asm_disable_page_table_write_protect();
    }

    // SAFETY: Disabling the write protection does not affect the memory safety.
    unsafe { asm_disable_page_table_write_protect() }
}
