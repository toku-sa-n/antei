pub mod array;
pub mod single;

use {
    super::{map, unmap},
    core::{convert::TryInto, num::NonZeroUsize},
    os_units::Bytes,
    x86_64::{structures::paging::PageTableFlags, PhysAddr, VirtAddr},
};

#[derive(Copy, Clone, Debug)]
pub struct Mapper;
impl accessor::Mapper for Mapper {
    unsafe fn map(&mut self, phys_start: usize, bytes: usize) -> NonZeroUsize {
        let p = PhysAddr::new(phys_start.try_into().unwrap());

        let b = Bytes::new(bytes);

        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        let v = unsafe { map(p, b, flags) };

        NonZeroUsize::new(v.as_u64().try_into().unwrap()).unwrap()
    }

    fn unmap(&mut self, virt_start: usize, bytes: usize) {
        let v = VirtAddr::new(virt_start.try_into().unwrap());

        let b = Bytes::new(bytes);

        unmap(v, b);
    }
}
