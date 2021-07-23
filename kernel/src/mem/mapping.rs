use {
    super::{frame_allocator, mapper},
    x86_64::structures::paging::{Mapper, Page, PageTableFlags, PhysFrame, Size4KiB},
};

/// # Safety
///
/// See: [`Mapper::map_to`].
pub unsafe fn map_to(page: Page<Size4KiB>, frame: PhysFrame<Size4KiB>, flags: PageTableFlags) {
    let f = unsafe { mapper().map_to(page, frame, flags, &mut *frame_allocator()) };
    let f = f.expect("Failed to map a page.");

    f.flush();
}

pub fn unmap(page: Page<Size4KiB>) {
    let r = mapper().unmap(page);

    let (_, f) = r.expect("Failed to unmap a page.");

    f.flush();
}
