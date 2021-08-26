#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod accessor;
mod map;
mod phys;

use {uefi::service::boot::MemoryDescriptor, x86_64::structures::paging::Size4KiB};

pub use {
    map::{copy_current_pml4, elf::map_elf, map, map_page_range_to_unused_frame_range, unmap},
    phys::frame_allocator,
};

pub(crate) type NumOfPages<T = Size4KiB> = os_units::NumOfPages<T>;

/// # Safety
///
/// When and after calling this function,
/// - the virtual address `0xff7f_bfdf_e000` must point to the current working PML4.
/// - any references must not point to the current working PML4.
pub unsafe fn init(mmap: &[MemoryDescriptor]) {
    phys::init(mmap);

    // SAFETY: The caller must uphold the safety requirements.
    unsafe {
        map::init();
    }
}
