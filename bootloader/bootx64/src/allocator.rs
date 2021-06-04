use uefi_wrapper::service::boot::MemoryDescriptor;
use uefi_wrapper::service::boot::MemoryType;
use x86_64::structures::paging::{PageSize, Size4KiB};
use x86_64::{
    structures::paging::{FrameAllocator, PhysFrame},
    PhysAddr,
};

struct Allocator<'a> {
    mem_map: &'a mut [MemoryDescriptor],
}
impl<'a> Allocator<'a> {
    fn new(mem_map: &'a mut [MemoryDescriptor]) -> Self {
        Self { mem_map }
    }

    fn iter_mut_conventional(&mut self) -> impl Iterator<Item = &mut MemoryDescriptor> {
        self.mem_map
            .iter_mut()
            .filter(|x| x.r#type == MemoryType::ConventionalMemory as u32)
    }
}
unsafe impl FrameAllocator<Size4KiB> for Allocator<'_> {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        for d in self.iter_mut_conventional() {
            if d.number_of_pages > 0 {
                let f = d.physical_start;
                let f = PhysAddr::new(f);
                let f = PhysFrame::from_start_address(f);
                let f = f.expect("The address is not page-aligned.");

                d.number_of_pages -= 1;
                d.physical_start += Size4KiB::SIZE;

                return Some(f);
            }
        }

        None
    }
}
