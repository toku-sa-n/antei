use {
    super::AddressSpace,
    aligned_ptr::ptr,
    core::convert::TryInto,
    elfloader::{ElfLoader, ElfLoaderErr, Flags, LoadableHeaders, ProgramHeader, Rela, VAddr, P64},
    os_units::{Bytes, NumOfPages},
    x86_64::{
        structures::paging::{
            page::PageRange, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
        },
        VirtAddr,
    },
};

pub(super) struct Loader<'a, T: FrameAllocator<Size4KiB>> {
    address_space: &'a mut AddressSpace,
    frame_allocator: &'a mut T,
}
impl<'a, T: FrameAllocator<Size4KiB>> Loader<'a, T> {
    pub(super) fn new(address_space: &'a mut AddressSpace, frame_allocator: &'a mut T) -> Self {
        Self {
            address_space,
            frame_allocator,
        }
    }

    fn allocate_for_header(&mut self, header: ProgramHeader) {
        let range = page_range_from_header(header);
        let flags =
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;

        let r = unsafe {
            self.address_space
                .map_range_to_unused(range, flags, self.frame_allocator)
        };
        r.expect("Failed to allocate a page.");
    }
}
impl<'a, T: FrameAllocator<Size4KiB>> ElfLoader for Loader<'a, T> {
    fn allocate(&mut self, load_headers: LoadableHeaders) -> Result<(), elfloader::ElfLoaderErr> {
        for header in load_headers {
            self.allocate_for_header(header);
        }

        Ok(())
    }

    fn load(&mut self, flags: Flags, base: VAddr, region: &[u8]) -> Result<(), ElfLoaderErr> {
        let base = VirtAddr::new(base);
        let base_page = Page::from_start_address(base);
        let base_page = base_page.expect("The address is not page-aligned.");

        let phys_base = self.address_space.translate_page(base_page);
        let phys_base = phys_base.expect("The page is not mapped.");

        let bytes = Bytes::new(region.len());

        let page_table_flags =
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;

        // SAFETY: This method assumes that the preceding `allocate()` call maps `bytes` bytes from
        // `phys_base` to the consecutive physical memory.
        let virt_base = unsafe { vm::map(phys_base.start_address(), bytes, page_table_flags) };

        unsafe {
            ptr::copy_nonoverlapping(region.as_ptr(), virt_base.as_mut_ptr(), region.len());
        }

        vm::unmap(virt_base, bytes);

        if !flags.is_write() {
            self.make_readonly(base.as_u64(), region.len())?;
        }

        Ok(())
    }

    fn relocate(&mut self, _entry: &Rela<P64>) -> Result<(), ElfLoaderErr> {
        todo!()
    }

    fn make_readonly(&mut self, base: VAddr, size: usize) -> Result<(), ElfLoaderErr> {
        let base = VirtAddr::new(base);

        let bytes = Bytes::new(size);

        let page_range = page_range_from_base_and_num_of_pages(base, bytes.as_num_of_pages());

        let r = unsafe {
            self.address_space.update_flags_for_range(
                page_range,
                PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE,
            )
        };
        r.expect("Failed to mark pages as readonly.");

        Ok(())
    }
}

fn page_range_from_header(header: ProgramHeader) -> PageRange {
    let start = header.virtual_addr();
    let start = VirtAddr::new(start);
    let start = Page::from_start_address(start);
    let start = start.expect("The address is not page-aligned.");

    let end = start.start_address() + header.mem_size();
    let end = Page::from_start_address(end);
    let end = end.expect("The address is not page-aligned.");

    PageRange { start, end }
}

fn page_range_from_base_and_num_of_pages(
    base: VirtAddr,
    num_of_pages: NumOfPages<Size4KiB>,
) -> PageRange {
    let start = Page::from_start_address(base);
    let start = start.expect("The address is not page-aligned.");

    let end = start + num_of_pages.as_usize().try_into().unwrap();

    PageRange { start, end }
}
