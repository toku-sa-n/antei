use {
    super::mapper,
    crate::NumOfPages,
    core::convert::TryInto,
    kernel_mmap::Region,
    x86_64::{
        structures::paging::{PageSize, Size4KiB, Translate},
        VirtAddr,
    },
};

#[must_use]
pub fn find_unused_pages_from_region(n: NumOfPages, r: &Region) -> VirtAddr {
    try_find_unused_pages_from_region(n, r).expect("Failed to allocate a virtual page.")
}

fn try_find_unused_pages_from_region(n: NumOfPages, r: &Region) -> Option<VirtAddr> {
    let mut cnt = 0;
    let mut start = None;

    for a in (r.start().as_u64()..r.end().as_u64())
        .step_by(Size4KiB::SIZE.try_into().unwrap())
        .map(VirtAddr::new)
    {
        if available(a) {
            if start.is_none() {
                start = Some(a);
            }

            cnt += 1;

            if cnt >= n.as_usize() {
                return start;
            }
        } else {
            cnt = 0;
            start = None;
        }
    }

    None
}

fn available(a: VirtAddr) -> bool {
    mapper().translate_addr(a).is_none() && !a.is_null()
}
