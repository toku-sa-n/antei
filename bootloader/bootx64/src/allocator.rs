use core::convert::TryFrom;
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
    pub(crate) fn new(mmap: &'a mut [MemoryDescriptor]) -> Self {
        Self { mmap }
    }

    fn allocate_frames(&mut self, n: NumOfPages<Size4KiB>) -> Option<PhysAddr> {
        self.iter_mut_conventional()
            .find_map(|d| Self::try_alloc_from(d, n))
    }

    fn iter_mut_conventional(&mut self) -> impl Iterator<Item = &mut MemoryDescriptor> {
        self.mmap.iter_mut().filter(|d| Self::is_usable_memory(d))
    }

    fn try_alloc_from(d: &mut MemoryDescriptor, n: NumOfPages<Size4KiB>) -> Option<PhysAddr> {
        (d.number_of_pages >= u64::try_from(n.as_usize()).unwrap()).then(|| Self::alloc_from(d, n))
    }

    fn alloc_from(d: &mut MemoryDescriptor, num_pages: NumOfPages<Size4KiB>) -> PhysAddr {
        let bytes = num_pages.as_bytes();
        let bytes: u64 = bytes.as_usize().try_into().unwrap();

        let num_pages: u64 = num_pages.as_usize().try_into().unwrap();

        let addr = d.physical_start;
        let addr = PhysAddr::new(addr);

        d.number_of_pages -= num_pages;
        d.physical_start += bytes;

        addr
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
