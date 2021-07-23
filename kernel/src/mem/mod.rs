mod phys;
mod pml4;

use uefi_wrapper::service::boot::MemoryDescriptor;

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
