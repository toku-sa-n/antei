use {
    crate::Allocator,
    aligned_ptr::ptr,
    x86_64::{
        structures::paging::{
            page::PageRange, FrameAllocator, Mapper as MapperTrait, Page, PageTableFlags,
            PhysFrame, RecursivePageTable, Size4KiB,
        },
        VirtAddr,
    },
};

const RECURSIVE_PAGING: VirtAddr = VirtAddr::new_truncate(0xffff_ff7f_bfdf_e000);

pub(crate) struct Mapper<'a> {
    allocator: &'a mut Allocator<'a>,
    mapper: RecursivePageTable<'a>,
}
impl<'a> Mapper<'a> {
    /// # Safety
    ///
    /// The caller must ensure that while the returned instance is alive
    /// - The recursive paging address `0xff7f_bfdf_e000` is accessible.
    /// - There is no reference to one of the all working page tables.
    pub(crate) unsafe fn new(allocator: &'a mut Allocator<'a>) -> Self {
        // SAFETY: The caller ensures that the recursive paging address is accessible.
        let table = unsafe { ptr::as_mut(RECURSIVE_PAGING.as_mut_ptr()) };

        let mapper = RecursivePageTable::new(table);
        let mapper = mapper.expect("The recursive entry is not enabled.");

        Self { allocator, mapper }
    }

    /// # Safety
    ///
    /// See [`x86_64::structures::paging::Mapper`].
    pub(crate) unsafe fn map_range_to_unused(&mut self, range: PageRange, flags: PageTableFlags) {
        for p in range {
            self.map_to_unused(p, flags);
        }
    }

    pub(crate) unsafe fn update_flags_for_range(
        &mut self,
        range: PageRange,
        flags: PageTableFlags,
    ) {
        for page in range {
            unsafe {
                self.update_flags(page, flags);
            }
        }
    }

    fn map_to_unused(&mut self, page: Page<Size4KiB>, flags: PageTableFlags) {
        let frame = self.allocator.allocate_frame();
        let frame = frame.expect("Physical frame is not available.");

        // SAFETY: The physical memory is not used by anyone.
        unsafe {
            self.map(page, frame, flags);
        }
    }

    /// # Safety
    ///
    /// See [`x86_64::structures::paging::Mapper`].
    unsafe fn map(&mut self, page: Page<Size4KiB>, frame: PhysFrame, flags: PageTableFlags) {
        let flush = unsafe { self.mapper.map_to(page, frame, flags, self.allocator) };
        let flush = flush.expect("Failed to map a page.");

        flush.flush();
    }

    unsafe fn update_flags(&mut self, page: Page<Size4KiB>, flags: PageTableFlags) {
        let r = unsafe { self.mapper.update_flags(page, flags) };
        let flush = r.expect("Failed to update flags.");

        flush.flush();
    }
}
