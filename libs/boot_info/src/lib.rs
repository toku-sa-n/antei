#![no_std]

use {aligned_ptr::slice, uefi_wrapper::service::boot::MemoryDescriptor, x86_64::VirtAddr};

const MAGIC_HEADER: u64 = 0x0114_0514_1919_0810;
const MAGIC_FOOTER: u64 = 0x0334_0072_dead_cafe;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BootInfo {
    magic_header: u64,

    mmap: Mmap,

    magic_footer: u64,
}
impl BootInfo {
    #[must_use]
    pub fn new(mmap: Mmap) -> Self {
        Self {
            magic_header: MAGIC_HEADER,

            mmap,

            magic_footer: MAGIC_FOOTER,
        }
    }

    pub fn check_header_and_footer(&self) {
        self.check_header();
        self.check_footer();
    }

    pub fn mmap_mut(&mut self) -> &mut Mmap {
        &mut self.mmap
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

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Mmap {
    start: VirtAddr,
    len: usize,
}
impl Mmap {
    /// # Safety
    ///
    /// An array of the type [`MemoryDescriptor`] whose len is `len` must exist from
    /// `start`.
    pub unsafe fn new(start: VirtAddr, len: usize) -> Self {
        Self { start, len }
    }

    pub fn as_slice_mut(&mut self) -> &mut [MemoryDescriptor] {
        // SAFETY: `BootInfo::new` ensures the safety.
        unsafe { slice::from_raw_parts_mut(self.start.as_mut_ptr(), self.len) }
    }
}
