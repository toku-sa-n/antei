use uefi_wrapper::service::boot::MemoryDescriptor;

struct Mapper<'a> {
    mem_map: &'a mut [MemoryDescriptor],
}
impl<'a> Mapper<'a> {
    fn new(mem_map: &'a mut [MemoryDescriptor]) -> Self {
        Self { mem_map }
    }
}
