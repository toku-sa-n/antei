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
    pub const fn bytes(&self) -> Bytes {
        Bytes::new(self.0.end - self.0.start)
    }

    const fn new(start: VirtAddr, bytes: Bytes) -> Self {
        let start = start.as_u64() as usize;

        let end = start + bytes.as_usize();

        Self(start..end)
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

pub const STACK: Region = Region::new(
    VirtAddr::new_truncate(0xffff_ffff_a000_0000),
    Bytes::new(4 * Size4KiB::SIZE as usize),
);

const_assert!(!KERNEL.overlaps_with(&STACK));

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
    fn bytes() {
        let bytes = Bytes::new(16);
        let r = Region::new(VirtAddr::zero(), bytes);

        assert_eq!(r.bytes(), bytes);
    }
}
