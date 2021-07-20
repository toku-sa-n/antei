#![no_std]

use {aligned_ptr::slice, uefi_wrapper::service::boot::MemoryDescriptor, x86_64::VirtAddr};

const MAGIC_HEADER: u64 = 0x0114_0514_1919_0810;
const MAGIC_FOOTER: u64 = 0x0334_0072_dead_cafe;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BootInfo {
    magic_header: u64,

    mmap_start: VirtAddr,
    mmap_len: usize,

    magic_footer: u64,
}
impl BootInfo {
    /// # Safety
    ///
    /// - An array of the type [`MemoryDescriptor`] whose len is `mmap_len` must exist from
    /// `mmap_start`.
    #[must_use]
    pub unsafe fn new(mmap_start: VirtAddr, mmap_len: usize) -> Self {
        Self {
            magic_header: MAGIC_HEADER,

            mmap_start,
            mmap_len,

            magic_footer: MAGIC_FOOTER,
        }
    }

    pub fn check_header_and_footer(&self) {
        self.check_header();
        self.check_footer();
    }

    pub fn mmap(&mut self) -> &mut [MemoryDescriptor] {
        // SAFETY: `BootInfo::new` ensures the safety.
        unsafe { slice::from_raw_parts_mut(self.mmap_start.as_mut_ptr(), self.mmap_len) }
    }

    fn check_header(&self) {
        assert_eq!(
            self.magic_header, MAGIC_HEADER,
            "Invalid boot information header."
        );
    }

    fn check_footer(&self) {
        assert_eq!(
            self.magic_footer, MAGIC_FOOTER,
            "Invalid boot information footer."
        );
    }
}
