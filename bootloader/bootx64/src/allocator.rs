use uefi_wrapper::service::boot::MemoryDescriptor;
use uefi_wrapper::service::boot::MemoryType;

struct Mapper<'a> {
    mem_map: &'a mut [MemoryDescriptor],
}
impl<'a> Mapper<'a> {
    fn new(mem_map: &'a mut [MemoryDescriptor]) -> Self {
        Self { mem_map }
    }

    fn iter_mut_conventional(&mut self) -> impl Iterator<Item = &mut MemoryDescriptor> {
        self.mem_map
            .iter_mut()
            .filter(|x| x.r#type == MemoryType::ConventionalMemory as u32)
    }
}
