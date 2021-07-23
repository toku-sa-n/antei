mod mapping;
mod phys;
mod pml4;
mod virt;

use {phys::frame_allocator, pml4::mapper, uefi_wrapper::service::boot::MemoryDescriptor};

pub use {
    mapping::{map_to, unmap},
    virt::find_unused_pages_from_region,
};

/// # Safety
///
/// When and after calling this function,
/// - the virtual address `0xff7f_bfdf_e000` must point to the current working PML4.
/// - any references must not point to the current working PML4.
pub unsafe fn init(mmap: &[MemoryDescriptor]) {
    phys::init(mmap);

    // SAFETY: The caller must uphold the safety requirements.
    unsafe { pml4::init() };
}
