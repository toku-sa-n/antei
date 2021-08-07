pub(crate) mod single;

use {
    super::virt,
    crate::NumOfPages,
    core::{
        convert::{TryFrom, TryInto},
        num::NonZeroUsize,
    },
    os_units::Bytes,
    x86_64::{
        structures::paging::{
            frame::PhysFrameRange, page::PageRange, Page, PageSize, PhysFrame, Size4KiB,
        },
        PhysAddr, VirtAddr,
    },
};

pub(crate) type Single<T> = accessor::Single<T, Mapper>;
pub(crate) type Array<T> = accessor::Array<T, Mapper>;

/// # Safety
///
/// Refer to [`accessor::Single::new`].
pub(crate) unsafe fn single<T: Copy>(p: PhysAddr) -> Single<T> {
    // SAFETY: The caller must uphold the safety requirements of the `new` method.
    unsafe { accessor::Single::new(p.as_u64().try_into().unwrap(), Mapper) }
}

/// # Safety
///
/// Refer to [`accessor::Array::new`].
pub(crate) unsafe fn array<T: Copy>(p: PhysAddr, len: usize) -> Array<T> {
    // SAFETY: The caller must uphold the safety requirements of the `new` method.
    unsafe { accessor::Array::new(p.as_u64().try_into().unwrap(), len, Mapper) }
}

pub(crate) struct Mapper;
impl Mapper {
    fn map_from_phys_addr_and_bytes(&self, p: PhysAddr, b: Bytes) -> VirtAddr {
        let frame_range = to_frame_range(p, b.as_num_of_pages());

        let page_range = unsafe {
            virt::map_frame_range_from_page_range(kernel_mmap::kernel_dma(), frame_range)
        };

        page_range.start.start_address() + p.as_u64() % Size4KiB::SIZE
    }
}
impl accessor::Mapper for Mapper {
    unsafe fn map(&mut self, phys_start: usize, bytes: usize) -> NonZeroUsize {
        let p = PhysAddr::new(phys_start.try_into().unwrap());

        let b = Bytes::new(bytes);

        let v = self.map_from_phys_addr_and_bytes(p, b);

        NonZeroUsize::new(v.as_u64().try_into().unwrap()).unwrap()
    }

    fn unmap(&mut self, virt_start: usize, bytes: usize) {
        let v = VirtAddr::new(virt_start.try_into().unwrap());
        let b = Bytes::new(bytes);

        unmap_from_virt_addr_and_bytes(v, b);
    }
}

fn unmap_from_virt_addr_and_bytes(v: VirtAddr, b: Bytes) {
    virt::unmap_range(to_page_range(v, b.as_num_of_pages()));
}

fn to_frame_range<S: PageSize>(p: PhysAddr, n: NumOfPages<S>) -> PhysFrameRange<S> {
    let start = PhysFrame::containing_address(p);

    let end = p + u64::try_from(n.as_bytes().as_usize()).unwrap();
    let end = end.align_up(S::SIZE);
    let end = PhysFrame::containing_address(end);

    PhysFrameRange { start, end }
}

fn to_page_range<S: PageSize>(v: VirtAddr, n: NumOfPages<S>) -> PageRange<S> {
    let start = Page::containing_address(v);

    let end = v + u64::try_from(n.as_bytes().as_usize()).unwrap();
    let end = end.align_up(S::SIZE);
    let end = Page::containing_address(end);

    PageRange { start, end }
}
