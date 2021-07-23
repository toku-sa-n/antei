use {
    super::{frame_allocator, mapper},
    x86_64::structures::paging::{Mapper, Page, PageTableFlags, PhysFrame},
};

/// # Safety
///
/// See: [`Mapper::map_to`].
pub unsafe fn map_to(page: Page, frame: PhysFrame, flags: PageTableFlags) {
    let f = unsafe { mapper().map_to(page, frame, flags, &mut *frame_allocator()) };
    let f = f.expect("Failed to map a page.");

    f.flush();
}

pub fn unmap(page: Page) {
    let r = mapper().unmap(page);

    let (_, f) = r.expect("Failed to unmap a page.");

    f.flush();
}
