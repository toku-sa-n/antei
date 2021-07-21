#![no_std]

use {
    aligned_ptr::slice,
    uefi_wrapper::service::boot::{
        MemoryDescriptor, MemoryType, ACPI_MEMORY_NVS, ACPI_RECLAIM_MEMORY, BOOT_SERVICES_CODE,
        BOOT_SERVICES_DATA, CONVENTIONAL_MEMORY, LOADER_CODE, LOADER_DATA, MEMORY_MAPPED_IO,
        MEMORY_MAPPED_IO_PORT_SPACE, MEMORY_MORE_RELIABLE, MEMORY_NV, MEMORY_RO, MEMORY_RP,
        MEMORY_RUNTIME, MEMORY_UC, MEMORY_UCE, MEMORY_WB, MEMORY_WC, MEMORY_WP, MEMORY_WT,
        MEMORY_XP, PAL_CODE, PERSISTENT_MEMORY, RESERVED_MEMORY_TYPE, RUNTIME_SERVICES_CODE,
        RUNTIME_SERVICES_DATA, UNUSABLE_MEMORY,
    },
    x86_64::{PhysAddr, VirtAddr},
};

const MAGIC_HEADER: u64 = 0x0114_0514_1919_0810;
const MAGIC_FOOTER: u64 = 0x0334_0072_dead_cafe;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BootInfo {
    magic_header: u64,

    mmap: Mmap,
    rsdp: PhysAddr,

    magic_footer: u64,
}
impl BootInfo {
    #[must_use]
    pub fn new(mmap: Mmap, rsdp: PhysAddr) -> Self {
        Self {
            magic_header: MAGIC_HEADER,

            mmap,
            rsdp,

            magic_footer: MAGIC_FOOTER,
        }
    }

    pub fn validate(&self) {
        self.check_header_and_footer();
        self.mmap.vaildate();
    }

    fn check_header_and_footer(&self) {
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
    #[must_use]
    pub unsafe fn new(start: VirtAddr, len: usize) -> Self {
        Self { start, len }
    }

    pub fn as_slice_mut(&mut self) -> &mut [MemoryDescriptor] {
        // SAFETY: `Mmap::new` ensures the safety.
        unsafe { slice::from_raw_parts_mut(self.start.as_mut_ptr(), self.len) }
    }

    fn as_slice(&self) -> &[MemoryDescriptor] {
        // SAFETY: `Mmap::new` ensures the safety.
        unsafe { slice::from_raw_parts(self.start.as_ptr(), self.len) }
    }

    fn vaildate(&self) {
        for d in self.as_slice() {
            DescriptorValidater::new(*d).validate();
        }
    }
}

struct DescriptorValidater(MemoryDescriptor);
impl DescriptorValidater {
    fn new(d: MemoryDescriptor) -> Self {
        Self(d)
    }

    fn validate(&self) {
        self.validate_type();
        self.validate_attribute();
    }

    fn validate_type(&self) {
        const TYPES: &[MemoryType] = &[
            RESERVED_MEMORY_TYPE,
            LOADER_CODE,
            LOADER_DATA,
            BOOT_SERVICES_CODE,
            BOOT_SERVICES_DATA,
            RUNTIME_SERVICES_CODE,
            RUNTIME_SERVICES_DATA,
            CONVENTIONAL_MEMORY,
            UNUSABLE_MEMORY,
            ACPI_RECLAIM_MEMORY,
            ACPI_MEMORY_NVS,
            MEMORY_MAPPED_IO,
            MEMORY_MAPPED_IO_PORT_SPACE,
            PAL_CODE,
            PERSISTENT_MEMORY,
        ];

        const MIN_CUSTOM_MEMORY_TYPE: MemoryType = 0x7000_0000;

        for ty in TYPES {
            if &self.0.r#type == ty {
                return;
            }
        }

        if self.0.r#type < MIN_CUSTOM_MEMORY_TYPE {
            panic!("Invalid memory type: 0x{:X}", self.0.r#type);
        }
    }

    fn validate_attribute(&self) {
        const ALL_ATTRIBUTES: u64 = MEMORY_UC
            | MEMORY_WC
            | MEMORY_WT
            | MEMORY_WB
            | MEMORY_UCE
            | MEMORY_WP
            | MEMORY_RP
            | MEMORY_XP
            | MEMORY_NV
            | MEMORY_MORE_RELIABLE
            | MEMORY_RO
            | MEMORY_RUNTIME;

        if self.0.attribute | ALL_ATTRIBUTES != ALL_ATTRIBUTES {
            panic!("Invalid memory attribute: 0x{:X}", self.0.attribute);
        }
    }
}
