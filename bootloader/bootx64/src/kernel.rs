use crate::elf;
use uefi_wrapper::service::boot::MemoryDescriptor;
use x86_64::VirtAddr;

/// # Safety
///
/// The caller must ensure that
/// - The recursive paging address `0xff7f_bfdf_e000` is accessible.
/// - There is no reference to one of the all working page tables.
pub unsafe fn load_and_jump(binary: &[u8], mmap: &mut [MemoryDescriptor]) -> ! {
    jump_to_kernel(unsafe { load(binary, mmap) });
}

/// # Safety
///
/// The caller must ensure that
/// - The recursive paging address `0xff7f_bfdf_e000` is accessible.
/// - There is no reference to one of the all working page tables.
unsafe fn load(binary: &[u8], mmap: &mut [MemoryDescriptor]) -> VirtAddr {
    // SAFETY: The caller upholds the safety requirements.
    let entry = unsafe { elf::load(binary, mmap) };

    assert!(!entry.is_null(), "The entry address is null.");

    entry
}

fn jump_to_kernel(entry: VirtAddr) -> ! {
    // SAFETY: Safe as described in
    // https://rust-lang.github.io/unsafe-code-guidelines/layout/function-pointers.html#representation.
    let entry: fn() -> ! = unsafe { core::mem::transmute(entry.as_ptr::<()>()) };

    (entry)()
}
