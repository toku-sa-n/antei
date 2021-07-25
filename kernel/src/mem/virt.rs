use {
    super::phys::frame_allocator,
    crate::NumOfPages,
    aligned_ptr::ptr,
    conquer_once::spin::OnceCell,
    core::convert::TryInto,
    kernel_mmap::Region,
    spinning_top::{MappedSpinlockGuard, Spinlock, SpinlockGuard},
    x86_64::{
        structures::paging::{
            Mapper, Page, PageSize, PageTable, PageTableFlags, PhysFrame, RecursivePageTable,
            Size4KiB, Translate,
        },
        PhysAddr, VirtAddr,
    },
};

const KERNEL_PAGE_FLAGS: PageTableFlags = PageTableFlags::from_bits_truncate(
    PageTableFlags::PRESENT.bits() | PageTableFlags::WRITABLE.bits(),
);

static PML4: OnceCell<Spinlock<RecursivePageTable<'_>>> = OnceCell::uninit();

/// # Safety
///
/// Hereafter,
/// - The recursive address `0xff7f_bfdf_e000` must point to the current working PML4.
/// - There must not exist any references that point to the current working PML4.
pub(super) unsafe fn init() {
    // SAFETY: The caller must uphold the safety requirement.
    unsafe { init_static() };

    unmap_all_user_regions();

    #[cfg(test_on_qemu)]
    tests::main();
}

pub(crate) fn map_frames_to_region(p: PhysAddr, n: NumOfPages, r: &Region) -> VirtAddr {
    assert!(
        p.is_aligned(Size4KiB::SIZE),
        "The address is not page-aligned."
    );

    let frames = to_frames(p, n);

    let start_v = find_unmapped_pages_from_region(n, r);
    let pages = to_pages(start_v, n);

    map_multiple_pages_and_frames_to(pages, frames);

    start_v
}

pub(crate) fn unmap_memory(v: VirtAddr, n: NumOfPages) {
    assert!(
        v.is_aligned(Size4KiB::SIZE),
        "The address is not page-aligned."
    );

    unmap_multiple_pages(to_pages(v, n));
}

/// # Safety
///
/// Hereafter, the virtual address `0xff7f_bfdf_e000` must point to the current working PML4.
unsafe fn init_static() {
    const RECURSIVE_ADDR: VirtAddr = VirtAddr::new_truncate(0xff7f_bfdf_e000);

    // SAFETY: The caller must ensure that the recursive paging address must point to the current
    // working PML4.
    let working_pml4 = unsafe { ptr::as_mut(RECURSIVE_ADDR.as_mut_ptr()) };
    let working_pml4 = RecursivePageTable::new(working_pml4);
    let working_pml4 =
        working_pml4.expect("Failed to get a reference to the current working PML4.");

    let r = PML4.try_init_once(|| Spinlock::new(working_pml4));
    r.expect("Failed to initialize a reference to PML4.");
}

fn to_frames(start: PhysAddr, n: NumOfPages) -> impl Iterator<Item = PhysFrame> + Clone {
    assert!(
        start.is_aligned(Size4KiB::SIZE),
        "The address is not page-aligned."
    );

    let end = start + n.as_bytes();
    assert!(
        end.is_aligned(Size4KiB::SIZE),
        "The address is not page-aligned."
    );

    (start.as_u64()..end.as_u64()).map(|a| PhysFrame::from_start_address(PhysAddr::new(a)).unwrap())
}

fn to_pages(start: VirtAddr, n: NumOfPages) -> impl Iterator<Item = Page> + Clone {
    assert!(
        start.is_aligned(Size4KiB::SIZE),
        "The address is not page-aligned."
    );

    let end = start + n.as_bytes();
    assert!(
        end.is_aligned(Size4KiB::SIZE),
        "The address is not page-aligned."
    );

    (start.as_u64()..end.as_u64())
        .step_by(Size4KiB::SIZE.try_into().unwrap())
        .map(|a| Page::from_start_address(VirtAddr::new(a)).unwrap())
}

fn map_multiple_pages_and_frames_to(
    pages: impl Iterator<Item = Page> + Clone,
    frames: impl Iterator<Item = PhysFrame> + Clone,
) {
    assert_eq!(
        pages.clone().count(),
        frames.clone().count(),
        "Slice lengths are not same."
    );

    for (p, f) in pages.zip(frames) {
        map_to(p, f);
    }
}

fn unmap_multiple_pages(pages: impl Iterator<Item = Page>) {
    for p in pages {
        unmap(p);
    }
}

fn map_to(page: Page, frame: PhysFrame) {
    let f = unsafe { mapper().map_to(page, frame, KERNEL_PAGE_FLAGS, &mut *frame_allocator()) };
    let f = f.expect("Failed to map a page.");

    f.flush();
}

fn unmap(page: Page) {
    let r = mapper().unmap(page);
    let (_, f) = r.expect("Failed to unmap a page.");

    f.flush();
}

fn find_unmapped_pages_from_region(n: NumOfPages, r: &Region) -> VirtAddr {
    try_find_unmapped_pages_from_region(n, r).expect("Pages unavailable.")
}

fn try_find_unmapped_pages_from_region(n: NumOfPages, r: &Region) -> Option<VirtAddr> {
    let start = r.start().as_u64();
    let end = r.end().as_u64();
    let addrs = (start..end).step_by(Size4KiB::SIZE.try_into().unwrap());
    let addrs = addrs.map(VirtAddr::new);

    let v = find_consecutive_satisfying_elements(addrs, is_available, n.as_usize())?;

    assert!(
        v.is_aligned(Size4KiB::SIZE),
        "The address is not page-aligned."
    );

    Some(v)
}

fn find_consecutive_satisfying_elements<T>(
    iter: impl Iterator<Item = T>,
    cond: impl Fn(&T) -> bool,
    n: usize,
) -> Option<T> {
    let mut cnt = 0;
    let mut start = None;

    for x in iter {
        if cond(&x) {
            cnt += 1;

            if start.is_none() {
                start = Some(x);
            }

            if cnt >= n {
                return start;
            }
        } else {
            cnt = 0;
            start = None;
        }
    }

    None
}

fn is_available(a: &VirtAddr) -> bool {
    mapper().translate_addr(*a).is_none() && !a.is_null()
}

fn unmap_all_user_regions() {
    let mut pml4 = pml4();

    for i in 0..510 {
        pml4[i].set_unused();
    }
}

fn pml4<'a>() -> MappedSpinlockGuard<'a, PageTable> {
    SpinlockGuard::map(mapper(), |m| m.level_4_table())
}

fn mapper<'a>() -> SpinlockGuard<'a, RecursivePageTable<'static>> {
    let pml4 = PML4.try_get();
    let pml4 = pml4.expect("`pml4::init` is not called.");
    let pml4 = pml4.try_lock();

    pml4.expect("Failed to acquire the lock of kernel's PML4.")
}

#[cfg(test_on_qemu)]
mod tests {
    use {
        super::{mapper, pml4},
        x86_64::{registers::control::Cr3, structures::paging::Translate, VirtAddr},
    };

    pub(super) fn main() {
        user_region_is_not_mapped();
        cr3_indicates_correct_pml4();
    }

    fn user_region_is_not_mapped() {
        let pml4 = pml4();

        for i in 0..510 {
            assert!(pml4[i].is_unused());
        }
    }

    fn cr3_indicates_correct_pml4() {
        let (current_pml4, _) = Cr3::read();
        let current_pml4_addr = current_pml4.start_address();

        let mut mapper = mapper();
        let expected_pml4 = mapper.level_4_table();
        let expected_pml4_addr = VirtAddr::from_ptr(expected_pml4);
        let expected_pml4_addr = mapper.translate_addr(expected_pml4_addr).unwrap();

        assert_eq!(current_pml4_addr, expected_pml4_addr);
    }
}
