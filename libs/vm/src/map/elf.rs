use {
    aligned_ptr::ptr,
    core::convert::TryInto,
    elfloader::{
        ElfBinary, ElfLoader, ElfLoaderErr, Flags, LoadableHeaders, ProgramHeader, Rela, VAddr, P64,
    },
    x86_64::{
        structures::paging::{page::PageRange, Page, PageSize, PageTableFlags},
        VirtAddr,
    },
};

/// # Safety
///
/// This function maps the given ELF binary to the current address space.
#[must_use]
#[allow(clippy::module_name_repetitions)]
pub unsafe fn map_elf(binary: &[u8]) -> VirtAddr {
    let elf = ElfBinary::new(binary);
    let elf = elf.expect("Not an ELF file.");

    let r = elf.load(&mut Loader);
    r.expect("Failed to map a ELF file.");

    VirtAddr::new(elf.entry_point())
}

struct Loader;
impl Loader {
    fn allocate_for_header(header: ProgramHeader<'_>) {
        if header.virtual_addr() != 0 {
            // SAFETY: Checked.
            unsafe {
                Self::allocate_for_header_unchecked(header);
            }
        }
    }

    /// # Safety
    ///
    /// `header.virtual_addr()` must not be 0.
    unsafe fn allocate_for_header_unchecked(header: ProgramHeader<'_>) {
        let page_range = Self::page_range_from_header(header);

        let flags =
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;

        unsafe {
            super::map_page_range_to_unused_frame_range(page_range, flags);
        }
    }

    fn page_range_from_header<S: PageSize>(header: ProgramHeader<'_>) -> PageRange<S> {
        Self::page_range_from_vaddr_and_len(
            header.virtual_addr(),
            header.mem_size().try_into().unwrap(),
        )
    }

    fn page_range_from_vaddr_and_len<S: PageSize>(base: VAddr, len: usize) -> PageRange<S> {
        let start = VirtAddr::new(base);

        let end = start + len;
        let end = end.align_up(S::SIZE);

        let start = Page::from_start_address(start);
        let start = start.expect("The address is not page-aligned.");

        let end = Page::containing_address(end);

        PageRange { start, end }
    }

    unsafe fn update_flags(page_range: PageRange, flags: Flags) {
        unsafe {
            super::update_flags_for_range(page_range, Self::elf_flags_to_page_table_flags(flags));
        }
    }

    fn elf_flags_to_page_table_flags(flags: Flags) -> PageTableFlags {
        let mut page_table_flags = PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE;

        if flags.is_write() {
            page_table_flags |= PageTableFlags::WRITABLE;
        }

        if !flags.is_execute() {
            page_table_flags |= PageTableFlags::NO_EXECUTE;
        }

        page_table_flags
    }
}
impl ElfLoader for Loader {
    fn allocate(&mut self, load_headers: LoadableHeaders<'_, '_>) -> Result<(), ElfLoaderErr> {
        for header in load_headers {
            Self::allocate_for_header(header);
        }

        Ok(())
    }

    fn load(&mut self, flags: Flags, base: VAddr, region: &[u8]) -> Result<(), ElfLoaderErr> {
        let base = VirtAddr::new(base);

        assert!(!base.is_null());

        unsafe {
            ptr::copy_nonoverlapping(region.as_ptr(), base.as_mut_ptr(), region.len());
        }

        let page_range = Self::page_range_from_vaddr_and_len(base.as_u64(), region.len());

        unsafe {
            Self::update_flags(page_range, flags);
        }

        Ok(())
    }

    fn relocate(&mut self, _entry: &Rela<P64>) -> Result<(), ElfLoaderErr> {
        todo!()
    }

    fn make_readonly(&mut self, base: VAddr, size: usize) -> Result<(), ElfLoaderErr> {
        let page_range = Self::page_range_from_vaddr_and_len(base, size);

        let flags = PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE;

        unsafe {
            super::update_flags_for_range(page_range, flags);
        }

        Ok(())
    }
}
