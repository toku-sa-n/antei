use crate::Allocator;
use aligned::ptr;
use x86_64::structures::paging::RecursivePageTable;
use x86_64::structures::paging::{self, Size4KiB};
use x86_64::VirtAddr;

const RECURSIVE_PAGING: VirtAddr = VirtAddr::new_truncate(0xffff_ff7f_bfdf_e000);

struct Mapper<'a> {
    allocator: &'a mut Allocator<'a>,
    mapper: RecursivePageTable<'a>,
}
impl<'a> Mapper<'a> {
    /// # Safety
    ///
    /// The caller must ensure that the recursive paging address 0xff7f_bfdf_e000 is accessible.
    unsafe fn new(allocator: &'a mut Allocator<'a>) -> Self {
        // SAFETY: The caller ensures that the recursive paging address is accessible.
        let table = unsafe { ptr::as_mut(RECURSIVE_PAGING.as_mut_ptr()) };
        let mapper = RecursivePageTable::new(table);
        let mapper = mapper.expect("The recursive entry is not enabled.");

        Self { allocator, mapper }
    }
}
