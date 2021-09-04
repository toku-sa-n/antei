use {
    crate::{fs, paging, Allocator, Mapper, NumOfPages, SystemTable},
    aligned_ptr::ptr,
    core::convert::TryInto,
    predefined_mmap::initrd,
    uefi::service::boot::MemoryDescriptor,
    x86_64::structures::paging::PageTableFlags as Flags,
    x86_64::structures::paging::Size4KiB,
};

pub fn locate<'a>(st: &mut SystemTable) -> &'a [u8] {
    fs::locate(st, "initrd.cpio")
}

/// # Safety
///
/// The caller must ensure that
/// - The virtual address `0xff7f_bfdf_e000` points to the current working PML4.
/// - Any references do not point to one of all working page tables.
#[cfg_attr(target_pointer_width = "64", allow(clippy::missing_panics_doc))]
pub unsafe fn map_and_load(binary: &[u8], mmap: &mut [MemoryDescriptor]) {
    ensure_initrd_is_small_enough(binary);

    let mut allocator = Allocator::new(mmap);

    // SAFETY: The caller must ensure that the address `0xff7f_bfdf_e000` points to the current
    // working PML4 and any references must not point to one of all working page tables.
    let mut mapper = unsafe { Mapper::new(&mut allocator) };

    paging::edit_page_tables(|| map_with_mapper(binary, &mut mapper));
}

fn ensure_initrd_is_small_enough(binary: &[u8]) {
    let initrd_pages: usize = (initrd().end - initrd().start).try_into().unwrap();
    let initrd_pages = NumOfPages::<Size4KiB>::new(initrd_pages);

    let initrd_bytes = initrd_pages.as_bytes();

    assert!(binary.len() < initrd_bytes.as_usize(), "Initrd is too big.");
}

fn map_with_mapper(binary: &[u8], mapper: &mut Mapper<'_>) {
    // SAFETY: The memory region is not used by others.
    unsafe {
        mapper.map_range_to_unused(initrd(), Flags::PRESENT | Flags::WRITABLE);

        ptr::copy_nonoverlapping(
            binary.as_ptr(),
            initrd().start.start_address().as_mut_ptr(),
            binary.len(),
        );

        mapper.update_flags_for_range(initrd(), Flags::PRESENT | Flags::GLOBAL | Flags::NO_EXECUTE);
    }
}
