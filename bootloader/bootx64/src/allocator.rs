use core::convert::TryInto;

use os_units::NumOfPages;
use uefi_wrapper::service::boot::MemoryDescriptor;
use uefi_wrapper::service::boot::MemoryType;
use x86_64::structures::paging::Size4KiB;
use x86_64::{
    structures::paging::{FrameAllocator, PhysFrame},
    PhysAddr,
};

pub(crate) struct Allocator<'a> {
    mmap: &'a mut [MemoryDescriptor],
}
impl<'a> Allocator<'a> {
    pub fn new(mem_map: &'a mut [MemoryDescriptor]) -> Self {
        Self { mmap: mem_map }
    }

    fn allocate_frames(&mut self, n: NumOfPages<Size4KiB>) -> Option<PhysAddr> {
        let bytes: u64 = n.as_bytes().as_usize().try_into().unwrap();
        let n: u64 = n.as_usize().try_into().unwrap();

        for d in self.iter_mut_conventional() {
            if d.number_of_pages >= n {
                let f = d.physical_start;
                let f = PhysAddr::new(f);

                d.number_of_pages -= n;
                d.physical_start += bytes;

                return Some(f);
            }
        }

        None
    }

    fn iter_mut_conventional(&mut self) -> impl Iterator<Item = &mut MemoryDescriptor> {
        self.mmap.iter_mut().filter(|d| Self::is_usable_memory(d))
    }

    fn is_usable_memory(d: &MemoryDescriptor) -> bool {
        d.r#type == MemoryType::ConventionalMemory as u32
    }
}
unsafe impl FrameAllocator<Size4KiB> for Allocator<'_> {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let f = self.allocate_frames(NumOfPages::new(1))?;
        let f = PhysFrame::from_start_address(f);

        Some(f.expect("The address is not page-aligned."))
    }
}
