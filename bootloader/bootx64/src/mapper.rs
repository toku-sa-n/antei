use crate::Allocator;
use aligned_ptr::ptr;
use os_units::NumOfPages;
use x86_64::structures::paging::PhysFrame;
use x86_64::structures::paging::RecursivePageTable;
use x86_64::structures::paging::Size4KiB;
use x86_64::structures::paging::{FrameAllocator, PageTableFlags};
use x86_64::structures::paging::{Mapper as MapperTrait, Page};
use x86_64::VirtAddr;

const RECURSIVE_PAGING: VirtAddr = VirtAddr::new_truncate(0xffff_ff7f_bfdf_e000);

pub(crate) struct Mapper<'a> {
    allocator: &'a mut Allocator<'a>,
    mapper: RecursivePageTable<'a>,
}
impl<'a> Mapper<'a> {
    /// # Safety
    ///
    /// The caller must ensure that the recursive paging address `0xff7f_bfdf_e000` is accessible.
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
    unsafe fn map(&mut self, page: Page<Size4KiB>, frame: PhysFrame, flags: PageTableFlags) {
        let flush = unsafe { self.mapper.map_to(page, frame, flags, self.allocator) };
        let flush = flush.expect("Failed to map a page.");
        flush.flush();
    }

    /// # Safety
    ///
    /// See [`x86_64::structures::paging::Mapper`].
    pub(crate) unsafe fn map_range_to_unused(
        &mut self,
        v: VirtAddr,
        n: NumOfPages<Size4KiB>,
        flags: PageTableFlags,
    ) {
        for i in 0..n.as_usize() {
            let i = NumOfPages::<Size4KiB>::new(i);

            let page = Page::from_start_address(v + i.as_bytes().as_usize());
            let page = page.expect("The address is not page-aligned.");

            self.map_to_unused(page, flags);
        }
    }

    pub(crate) unsafe fn update_flags_for_range(
        &mut self,
        v: VirtAddr,
        n: NumOfPages<Size4KiB>,
        flags: PageTableFlags,
    ) {
        for i in 0..n.as_usize() {
            let i = NumOfPages::<Size4KiB>::new(i);

            let page = Page::from_start_address(v + i.as_bytes().as_usize());
            let page = page.expect("The address is not page-aligned.");

            unsafe { self.update_flags(page, flags) };
        }
    }

    pub(crate) unsafe fn update_flags(&mut self, page: Page<Size4KiB>, flags: PageTableFlags) {
        let r = unsafe { self.mapper.update_flags(page, flags) };
        let flush = r.expect("Failed to update flags.");
        flush.flush();
    }

    fn map_to_unused(&mut self, page: Page<Size4KiB>, flags: PageTableFlags) {
        let frame = self.allocator.allocate_frame();
        let frame = frame.expect("Physical frame is not available.");

        // SAFETY: The physical memory is not used by anyone.
        unsafe { self.map(page, frame, flags) };
    }
}
