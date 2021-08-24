use {
    crate::{allocator::Allocator, paging, Mapper},
    aligned_ptr::ptr,
    core::convert::{TryFrom, TryInto},
    elfloader::{ElfBinary, ElfLoader, ElfLoaderErr, LoadableHeaders, ProgramHeader, VAddr},
    os_units::{Bytes, NumOfPages},
    uefi::service::boot::MemoryDescriptor,
    x86_64::{
        structures::paging::{
            page::{AddressNotAligned, PageRange},
            Page, PageSize, PageTableFlags,
        },
        VirtAddr,
    },
};

/// # Safety
///
/// The caller must ensure that
/// - The recursive paging address `0xff7f_bfdf_e000` is accessible.
/// - There is no reference to one of the all working page tables.
pub unsafe fn load(binary: &[u8], mmap: &mut [MemoryDescriptor]) -> VirtAddr {
    // SAFETY: The all rules are satisfied.
    paging::edit_page_tables(|| unsafe {
        let _ = &binary;

        load_without_disabling_page_table_protects(binary, mmap)
    })
}

/// # Safety
///
/// The caller must ensure that
/// - The recursive paging address `0xff7f_bfdf_e000` is accessible.
/// - There is no reference to one of the all working page tables.
unsafe fn load_without_disabling_page_table_protects(
    binary: &[u8],
    mmap: &mut [MemoryDescriptor],
) -> VirtAddr {
    let mut allocator = Allocator::new(mmap);

    // SAFETY: The caller ensures that the recursive paging is enabled and there is no reference to
    // one of the all working page tables.
    let mut mapper = unsafe { Mapper::new(&mut allocator) };

    let mut loader = Loader::new(&mut mapper);

    let elf = ElfBinary::new(binary);
    let elf = elf.expect("Not a ELF file.");

    let r = elf.load(&mut loader);
    r.expect("Failed to load a ELF file.");

    VirtAddr::new(elf.entry_point())
}

struct Loader<'a> {
    mapper: &'a mut Mapper<'a>,
}
impl<'a> Loader<'a> {
    fn new(mapper: &'a mut Mapper<'a>) -> Self {
        Self { mapper }
    }

    fn allocate_for_header(&mut self, h: ProgramHeader<'_>) {
        let v = VirtAddr::new(h.virtual_addr());

        let bytes = Bytes::new(h.mem_size().try_into().unwrap());

        let range = page_range_from_start_and_num(v, bytes.as_num_of_pages());
        let range = range.expect("The address is not page-aligned.");

        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        // SAFETY: The page will be used to load the ELF file. The memory does not have to be
        // initialized.
        unsafe {
            self.mapper.map_range_to_unused(range, flags);
        }

        // SAFETY: `bytes` from `v` are allocated by `map_range_to_unused`.
        unsafe {
            write_zeros(v.as_mut_ptr(), bytes);
        }
    }
}
impl ElfLoader for Loader<'_> {
    fn allocate(&mut self, load_headers: LoadableHeaders<'_, '_>) -> Result<(), ElfLoaderErr> {
        for h in load_headers {
            self.allocate_for_header(h);
        }

        Ok(())
    }

    fn load(
        &mut self,
        flags: elfloader::Flags,
        base: VAddr,
        region: &[u8],
    ) -> Result<(), ElfLoaderErr> {
        let base = VirtAddr::new(base);

        // SAFETY: Memory is allocated by the previous `allocate` call.
        unsafe {
            ptr::copy(region.as_ptr(), base.as_mut_ptr(), region.len());
        }

        if !flags.is_write() {
            let r = self.make_readonly(base.as_u64(), region.len());
            r.expect("Failed to make a region readonly.");
        }

        Ok(())
    }

    fn relocate(&mut self, _: &elfloader::Rela<elfloader::P64>) -> Result<(), ElfLoaderErr> {
        unimplemented!("The kernel must not have a relocation section.")
    }

    fn make_readonly(&mut self, base: VAddr, size: usize) -> Result<(), ElfLoaderErr> {
        let base = VirtAddr::new(base);

        let bytes = Bytes::new(size);

        let n = bytes.as_num_of_pages();

        let range = page_range_from_start_and_num(base, n);
        let range = range.expect("Address is not aligned.");

        unsafe {
            self.mapper
                .update_flags_for_range(range, PageTableFlags::PRESENT);
        }
        Ok(())
    }
}

/// # Safety
///
/// `start` must be valid for writes of `bytes`.
unsafe fn write_zeros(start: *mut u8, bytes: Bytes) {
    unsafe {
        ptr::write_bytes(start, 0, bytes.as_usize());
    }
}

fn page_range_from_start_and_num<S: PageSize>(
    v: VirtAddr,
    n: NumOfPages<S>,
) -> Result<PageRange<S>, AddressNotAligned> {
    let start = Page::from_start_address(v)?;

    let end = start + u64::try_from(n.as_usize()).unwrap();

    Ok(PageRange { start, end })
}
