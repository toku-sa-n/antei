use {
    crate::mem,
    acpi::AcpiHandler,
    core::{convert::TryInto, ptr::NonNull},
    kernel_mmap::KERNEL_DMA,
    os_units::Bytes,
    x86_64::PhysAddr,
};

#[derive(Copy, Clone, Debug)]
struct Handler;
impl AcpiHandler for Handler {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> acpi::PhysicalMapping<Self, T> {
        let p = PhysAddr::new(physical_address.try_into().unwrap());
        let n = Bytes::new(size).as_num_of_pages();

        let v = mem::map_frames_to_region(p, n, &KERNEL_DMA);

        // SAFETY: `n` pages from `v` is allocated by the previous `map_frames_to_region` call.
        // `region_length == size_of::<T>()`.
        unsafe {
            acpi::PhysicalMapping::new(
                physical_address,
                NonNull::new(v.as_mut_ptr()).unwrap(),
                core::mem::size_of::<T>(),
                n.as_bytes().as_usize(),
                *self,
            )
        }
    }

    fn unmap_physical_region<T>(region: &acpi::PhysicalMapping<Self, T>) {
        todo!()
    }
}
