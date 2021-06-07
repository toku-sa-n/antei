use aligned_ptr::ptr;
use x86_64::registers::control::Cr0;
use x86_64::registers::control::Cr0Flags;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::PageTable;
use x86_64::structures::paging::PageTableFlags;
use x86_64::PhysAddr;
use x86_64::VirtAddr;

pub fn enable_write_protect() {
    // SAFETY: Disabling the write protection does not affect the memory safety.
    unsafe { Cr0::update(|flags| flags.insert(Cr0Flags::WRITE_PROTECT)) }
}

pub fn disable_write_protect() {
    // SAFETY: Disabling the write protection does not affect the memory safety.
    unsafe { Cr0::update(|flags| flags.remove(Cr0Flags::WRITE_PROTECT)) }
}

/// # Safety
///
/// This function assumes that the physical and the virtual of the PML4 is the same value.
pub unsafe fn enable_recursive_paging() {
    let p = pml4_addr();
    let v = VirtAddr::new(p.as_u64());

    // SAFETY: The caller must ensure that the physical and virtual addresses of the PML4 is the
    // same value.
    let table: &mut PageTable = unsafe { ptr::as_mut(v.as_mut_ptr()) };

    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    table[0x510].set_addr(p, flags);
}

pub fn pml4_addr() -> PhysAddr {
    let f = Cr3::read().0;
    f.start_address()
}
