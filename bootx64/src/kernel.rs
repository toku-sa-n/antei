use {
    crate::{elf, fs, SystemTable},
    boot_info::{BootInfo, Mmap},
    uefi::service::boot::MemoryDescriptor,
    x86_64::{PhysAddr, VirtAddr},
};

pub fn locate<'a>(st: &mut SystemTable) -> &'a [u8] {
    fs::locate(st, "kernel")
}

/// # Safety
///
/// The caller must ensure that
/// - The recursive paging address `0xff7f_bfdf_e000` is accessible.
/// - There is no reference to one of the all working page tables.
pub unsafe fn load_and_jump(binary: &[u8], mmap: &mut [MemoryDescriptor], rsdp: PhysAddr) -> ! {
    jump(unsafe { load(binary, mmap) }, mmap, rsdp);
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

fn jump(entry: VirtAddr, mmap: &mut [MemoryDescriptor], rsdp: PhysAddr) -> ! {
    extern "sysv64" {
        fn switch_stack_and_call_kernel_code(
            boot_info: *mut BootInfo,
            entry: VirtAddr,
            stack_ptr: VirtAddr,
        ) -> !;
    }

    let mmap_start = VirtAddr::from_ptr(mmap.as_ptr());
    let mmap_len = mmap.len();

    // SAFETY: The pointer and the length are the correct ones.
    let mmap = unsafe { Mmap::new(mmap_start, mmap_len) };

    let mut boot_info = BootInfo::new(mmap, rsdp);

    // SAFETY: Correct arguments.
    unsafe {
        switch_stack_and_call_kernel_code(
            &mut boot_info,
            entry,
            predefined_mmap::stack().end.start_address(),
        )
    };
}
