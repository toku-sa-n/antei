use {
    super::phys,
    crate::NumOfPages,
    aligned_ptr::ptr,
    conquer_once::spin::OnceCell,
    core::convert::{TryFrom, TryInto},
    os_units::Bytes,
    spinning_top::{MappedSpinlockGuard, Spinlock, SpinlockGuard},
    x86_64::{
        structures::paging::{
            frame::PhysFrameRange, page::PageRange, Mapper, Page, PageSize, PageTable,
            PageTableFlags, PhysFrame, RecursivePageTable, Size4KiB, Translate,
        },
        PhysAddr, VirtAddr,
    },
};

pub(super) mod elf;

static PML4: OnceCell<Spinlock<RecursivePageTable<'_>>> = OnceCell::uninit();

/// # Safety
///
/// Refer to [`Mapper::map_to`].
#[must_use]
pub unsafe fn map(p: PhysAddr, b: Bytes, flags: PageTableFlags) -> VirtAddr {
    let frame_range = to_frame_range(p, b.as_num_of_pages());

    let page_range = unsafe { map_frame_range(frame_range, flags) };

    page_range.start.start_address() + p.as_u64() % Size4KiB::SIZE
}

pub fn unmap(v: VirtAddr, b: Bytes) {
    unmap_range(to_page_range(v, b.as_num_of_pages()));
}

#[must_use]
pub fn alloc_pages(n: NumOfPages, flags: PageTableFlags) -> Option<PageRange> {
    let all_range = PageRange {
        start: Page::containing_address(VirtAddr::zero()),
        end: predefined_mmap::kernel().start,
    };

    let page_range = find_unused_page_range_from_range(n, all_range)?;

    let frame_range = phys::frame_allocator().alloc(n)?;

    unsafe {
        map_range(page_range, frame_range, flags);
    }

    Some(page_range)
}

#[must_use]
pub fn current_pml4() -> PageTable {
    pml4().clone()
}

#[must_use]
pub fn translate(addr: VirtAddr) -> Option<PhysAddr> {
    mapper().translate_addr(addr)
}

/// # Safety
///
/// Hereafter,
/// - The recursive address `0xff7f_bfdf_e000` must point to the current working PML4.
/// - There must not exist any references that point to the current working PML4.
pub(super) unsafe fn init() {
    // SAFETY: The caller must uphold the safety requirement.
    unsafe {
        init_static();
    }

    unmap_all_user_regions();

    #[cfg(test_on_qemu)]
    tests::main();
}

pub(super) unsafe fn map_frame_range(
    frame_range: PhysFrameRange,
    flags: PageTableFlags,
) -> PageRange {
    unsafe { map_frame_range_from_page_range(predefined_mmap::kernel_dma(), frame_range, flags) }
}

pub(super) fn unmap_range(page_range: PageRange) {
    page_range.into_iter().for_each(unmap_page);
}

pub(super) unsafe fn map_page_range_to_unused_frame_range(
    page_range: PageRange,
    flags: PageTableFlags,
) {
    let frame_range = phys::frame_allocator().alloc(num_of_page_in_range(page_range));
    let frame_range = frame_range.expect("Frame not available.");

    unsafe {
        map_range(page_range, frame_range, flags);
    }
}

unsafe fn update_flags_for_range(page_range: PageRange, flags: PageTableFlags) {
    for page in page_range {
        unsafe {
            update_flags(page, flags);
        }
    }
}

unsafe fn map_frame_range_from_page_range(
    page_range: PageRange,
    frame_range: PhysFrameRange,
    flags: PageTableFlags,
) -> PageRange {
    unsafe {
        try_map_frame_range_from_page_range(page_range, frame_range, flags)
            .expect("Failed to map the physical frame range.")
    }
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

unsafe fn try_map_frame_range_from_page_range(
    page_range: PageRange,
    frame_range: PhysFrameRange,
    flags: PageTableFlags,
) -> Option<PageRange> {
    let n = frame_range.end - frame_range.start;
    let n = NumOfPages::new(n.try_into().unwrap());

    find_unused_page_range_from_range(n, page_range).map(|page_range| {
        unsafe {
            map_range(page_range, frame_range, flags);
        }
        page_range
    })
}

unsafe fn map_range(page_range: PageRange, frame_range: PhysFrameRange, flags: PageTableFlags) {
    for (p, f) in page_range.into_iter().zip(frame_range.into_iter()) {
        unsafe {
            map_page(p, f, flags);
        }
    }
}

unsafe fn map_page(page: Page, frame: PhysFrame, flags: PageTableFlags) {
    let f = unsafe { mapper().map_to(page, frame, flags, &mut *phys::frame_allocator()) };
    let f = f.expect("Failed to map a page.");

    f.flush();
}

fn unmap_page(page: Page) {
    let r = mapper().unmap(page);

    let (_, f) = r.expect("Failed to unmap a page.");

    f.flush();
}

unsafe fn update_flags(page: Page, flags: PageTableFlags) {
    let f = unsafe { mapper().update_flags(page, flags) };
    let f = f.expect("Failed to update page table flags.");

    f.flush();
}

fn find_unused_page_range_from_range(n: NumOfPages, range: PageRange) -> Option<PageRange> {
    let mut cnt = 0;
    let mut start = None;

    for p in range {
        if page_available(p) {
            if start.is_none() {
                start = Some(p);
            }

            cnt += 1;

            if cnt == n.as_usize() {
                return start.map(|start| {
                    let end = start + u64::try_from(cnt).unwrap();

                    PageRange { start, end }
                });
            }
        } else {
            cnt = 0;
            start = None;
        }
    }

    None
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

fn page_available(p: Page) -> bool {
    addr_available(p.start_address())
}

fn addr_available(a: VirtAddr) -> bool {
    mapper().translate_addr(a).is_none() && !a.is_null()
}

fn num_of_page_in_range<S: PageSize>(page_range: PageRange<S>) -> NumOfPages<S> {
    let n = page_range.end - page_range.start;

    NumOfPages::new(n.try_into().unwrap())
}

#[cfg(test_on_qemu)]
mod tests {
    use {
        super::{map, mapper, phys, pml4, unmap},
        crate::NumOfPages,
        os_units::Bytes,
        x86_64::{
            registers::control::Cr3,
            structures::paging::{PageTableFlags, Size4KiB, Translate},
            PhysAddr, VirtAddr,
        },
    };

    pub(super) fn main() {
        user_region_is_not_mapped();
        cr3_indicates_correct_pml4();
        map_and_unmap_page_aligned();
        map_and_unmap_over_page_boundary();
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

    fn map_and_unmap_page_aligned() {
        test_map_and_unmap(Bytes::zero(), Bytes::new(4));
    }

    fn map_and_unmap_over_page_boundary() {
        test_map_and_unmap(
            NumOfPages::<Size4KiB>::new(1).as_bytes() - Bytes::new(4),
            Bytes::new(8),
        );
    }

    fn test_map_and_unmap(offset: Bytes, map_size: Bytes) {
        let frame = phys::frame_allocator()
            .alloc((offset + map_size).as_num_of_pages())
            .unwrap();

        let phys = frame.start.start_address() + offset;

        let flags = PageTableFlags::PRESENT;

        let virt = unsafe { map(phys, map_size, flags) };

        assert_eq!(translate(virt), Some(phys));

        unmap(virt, map_size);

        assert_eq!(translate(virt), None);
    }

    fn translate(v: VirtAddr) -> Option<PhysAddr> {
        mapper().translate_addr(v)
    }
}
