use {
    crate::{Allocator, Mapper},
    uefi::service::boot::MemoryDescriptor,
    x86_64::structures::paging::PageTableFlags,
};

/// # Safety
///
/// - The virtual address `0xff7f_bfdf_e000` must point to the current working PML4.
/// - Any references must not point to one of all working page tables.
pub unsafe fn allocate(mmap: &mut [MemoryDescriptor]) {
    let mut allocator = Allocator::new(mmap);

    // SAFETY: The caller must uphold the safety requirements.
    let mut mapper = unsafe { Mapper::new(&mut allocator) };

    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    // SAFETY: The stack region is so high that nothing point to the region. There is no need to
    // worry about creating multiple mutable references.
    unsafe {
        mapper.map_range_to_unused(kernel_mmap::stack(), flags);
    }
}
