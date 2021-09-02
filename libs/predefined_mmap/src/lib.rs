#![no_std]

use {
    core::convert::TryFrom,
    os_units::{Bytes, NumOfPages},
    x86_64::{
        structures::paging::{page::PageRange, Page, PageSize},
        VirtAddr,
    },
};

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn kernel() -> PageRange {
    let start = VirtAddr::new(0xffff_ffff_8000_0000);

    let bytes = Bytes::new(0x2000_0000);

    let end = start + bytes;

    let start = Page::from_start_address(start).unwrap();

    let end = Page::from_start_address(end).unwrap();

    PageRange { start, end }
}

#[must_use]
pub fn stack() -> PageRange {
    next_to(kernel(), NumOfPages::new(32))
}

#[must_use]
pub fn kernel_dma() -> PageRange {
    next_to(stack(), NumOfPages::new(64))
}

#[must_use]
pub fn heap() -> PageRange {
    next_to(kernel_dma(), NumOfPages::new(64))
}

#[must_use]
pub fn for_testing() -> PageRange {
    next_to(heap(), NumOfPages::new(16))
}

#[must_use]
pub fn initrd() -> PageRange {
    next_to(for_testing(), NumOfPages::new(4))
}

fn next_to<S: PageSize>(range: PageRange<S>, n: NumOfPages<S>) -> PageRange<S> {
    let start = range.end;

    let end = start + u64::try_from(n.as_usize()).unwrap();

    PageRange { start, end }
}
