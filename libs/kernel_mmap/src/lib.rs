#![no_std]

use {
    core::ops::Range,
    os_units::Bytes,
    static_assertions::const_assert,
    x86_64::{
        structures::paging::{PageSize, Size4KiB},
        VirtAddr,
    },
};

#[derive(Clone, Debug)]
pub struct Region(Range<usize>);
impl Region {
    #[must_use]
    pub const fn start(&self) -> VirtAddr {
        VirtAddr::new_truncate(self.0.start as u64)
    }

    #[must_use]
    pub const fn end(&self) -> VirtAddr {
        VirtAddr::new_truncate(self.start().as_u64() + self.bytes().as_usize() as u64)
    }

    #[must_use]
    pub const fn bytes(&self) -> Bytes {
        Bytes::new(self.0.end - self.0.start)
    }

    const fn new(start: VirtAddr, bytes: Bytes) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        let start = start.as_u64() as usize;

        let end = start + bytes.as_usize();

        Self(start..end)
    }

    const fn next_to(other: &Region, bytes: Bytes) -> Self {
        Self::new(other.end(), bytes)
    }

    #[allow(dead_code)]
    const fn overlaps_with(&self, other: &Region) -> bool {
        self.0.end > other.0.start && other.0.end > self.0.start
    }
}

pub const KERNEL: Region = Region::new(
    VirtAddr::new_truncate(0xffff_ffff_8000_0000),
    Bytes::new(0x2000_0000),
);

#[allow(clippy::cast_possible_truncation)]
pub const STACK: Region = Region::next_to(&KERNEL, Bytes::new(8 * Size4KiB::SIZE as usize));

#[allow(clippy::cast_possible_truncation)]
pub const KERNEL_DMA: Region = Region::next_to(&STACK, Bytes::new(64 * Size4KiB::SIZE as usize));

#[allow(clippy::cast_possible_truncation)]
pub const FOR_TESTING: Region =
    Region::next_to(&KERNEL_DMA, Bytes::new(16 * Size4KiB::SIZE as usize));

const_assert!(!KERNEL.overlaps_with(&STACK));
const_assert!(!STACK.overlaps_with(&KERNEL_DMA));
const_assert!(!KERNEL_DMA.overlaps_with(&FOR_TESTING));

#[cfg(test)]
mod tests {
    use {super::Region, os_units::Bytes, x86_64::VirtAddr};

    #[test]
    fn range_overlaps() {
        let r1 = Region::new(VirtAddr::new(0x1000), Bytes::new(10));
        let r2 = Region::new(VirtAddr::new(0x1005), Bytes::new(20));

        assert!(r1.overlaps_with(&r2));
        assert!(r2.overlaps_with(&r1));
    }

    #[test]
    fn same_range() {
        let r1 = Region::new(VirtAddr::new(0x1000), Bytes::new(5));
        let r2 = r1.clone();

        assert!(r1.overlaps_with(&r2));
        assert!(r2.overlaps_with(&r1));
    }

    #[test]
    fn not_overlap() {
        let r1 = Region::new(VirtAddr::new(0x1000), Bytes::new(10));
        let r2 = Region::new(VirtAddr::new(0x2000), Bytes::new(10));

        assert!(!r1.overlaps_with(&r2));
        assert!(!r2.overlaps_with(&r1));
    }

    #[test]
    fn consecutive() {
        let r1 = Region::new(VirtAddr::new(0x1000), Bytes::new(16));
        let r2 = Region::new(VirtAddr::new(0x1010), Bytes::new(16));

        assert!(!r1.overlaps_with(&r2));
        assert!(!r2.overlaps_with(&r1));
    }

    #[test]
    fn including() {
        let r1 = Region::new(VirtAddr::new(0x1000), Bytes::new(16));
        let r2 = Region::new(VirtAddr::new(0x1008), Bytes::new(4));

        assert!(r1.overlaps_with(&r2));
        assert!(r2.overlaps_with(&r1));
    }

    #[test]
    fn start() {
        let start = VirtAddr::new(0xffff_ffff_8000_0000);
        let r = Region::new(start, Bytes::new(0x2000));

        assert_eq!(r.start(), start);
    }

    #[test]
    fn bytes() {
        let bytes = Bytes::new(16);
        let r = Region::new(VirtAddr::zero(), bytes);

        assert_eq!(r.bytes(), bytes);
    }

    #[test]
    fn next_to() {
        let r1 = Region::new(VirtAddr::new(0x1000), Bytes::new(0x1000));
        let r2 = Region::next_to(&r1, Bytes::new(0x1000));

        assert_eq!(r2.start(), VirtAddr::new(0x2000));
        assert!(!r1.overlaps_with(&r2));
    }
}
