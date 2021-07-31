#[cfg(test_on_qemu)]
use {
    super::phys,
    crate::NumOfPages,
    core::convert::{TryFrom, TryInto},
    x86_64::structures::paging::{
        frame::PhysFrameRange, page::PageRange, Mapper, Page, PageTableFlags, PhysFrame, Translate,
    },
};
use {
    aligned_ptr::ptr,
    conquer_once::spin::OnceCell,
    spinning_top::{MappedSpinlockGuard, Spinlock, SpinlockGuard},
    x86_64::{
        structures::paging::{PageTable, RecursivePageTable},
        VirtAddr,
    },
};

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

#[cfg(test_on_qemu)]
unsafe fn map_frame_range_from_page_range(
    page_range: PageRange,
    frame_range: PhysFrameRange,
) -> PageRange {
    unsafe {
        try_map_frame_range_from_page_range(page_range, frame_range)
            .expect("Failed to map the physical frame range.")
    }
}

#[cfg(test_on_qemu)]
unsafe fn try_map_frame_range_from_page_range(
    page_range: PageRange,
    frame_range: PhysFrameRange,
) -> Option<PageRange> {
    let n = frame_range.end - frame_range.start;
    let n = NumOfPages::new(n.try_into().unwrap());

    find_unused_page_range_from_range(n, page_range).map(|page_range| {
        unsafe { map_range(page_range, frame_range) };
        page_range
    })
}

#[cfg(test_on_qemu)]
unsafe fn map_range(page_range: PageRange, frame_range: PhysFrameRange) {
    for (p, f) in page_range.into_iter().zip(frame_range.into_iter()) {
        unsafe { map(p, f) };
    }
}

#[cfg(test_on_qemu)]
fn unmap_range(page_range: PageRange) {
    page_range.into_iter().for_each(unmap);
}

#[cfg(test_on_qemu)]
unsafe fn map(page: Page, frame: PhysFrame) {
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    let f = unsafe { mapper().map_to(page, frame, flags, &mut *phys::frame_allocator()) };
    let f = f.expect("Failed to map a page.");

    f.flush();
}

#[cfg(test_on_qemu)]
fn unmap(page: Page) {
    let r = mapper().unmap(page);

    let (_, f) = r.expect("Failed to unmap a page.");

    f.flush();
}

#[cfg(test_on_qemu)]
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

#[cfg(test_on_qemu)]
fn page_available(p: Page) -> bool {
    addr_available(p.start_address())
}

#[cfg(test_on_qemu)]
fn addr_available(a: VirtAddr) -> bool {
    mapper().translate_addr(a).is_none() && !a.is_null()
}

#[cfg(test_on_qemu)]
mod tests {
    use {
        super::{
            map, map_frame_range_from_page_range, map_range, mapper, phys, pml4,
            try_map_frame_range_from_page_range, unmap, unmap_range,
        },
        crate::NumOfPages,
        core::convert::TryInto,
        x86_64::{
            registers::control::Cr3,
            structures::paging::{FrameAllocator, Translate},
            VirtAddr,
        },
    };

    pub(super) fn main() {
        user_region_is_not_mapped();
        cr3_indicates_correct_pml4();
        map_and_unmap();
        map_and_unmap_range();
        map_frame_range_from_page_range_and_unmap();
        map_frame_range_from_page_range_fail();
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

    fn map_and_unmap() {
        let frame = phys::frame_allocator().allocate_frame().unwrap();

        let page = kernel_mmap::for_testing().start;

        unsafe { map(page, frame) };

        assert_eq!(
            mapper().translate_addr(page.start_address()),
            Some(frame.start_address())
        );

        unmap(page);

        assert_eq!(mapper().translate_addr(page.start_address()), None);
    }

    fn map_and_unmap_range() {
        let num = kernel_mmap::for_testing().end - kernel_mmap::for_testing().start;
        let num = NumOfPages::new(num.try_into().unwrap());

        let frames = phys::frame_allocator().alloc(num).unwrap();

        let pages = kernel_mmap::for_testing();

        unsafe {
            map_range(pages, frames);
        }

        for (p, f) in pages.into_iter().zip(frames.into_iter()) {
            assert_eq!(
                mapper().translate_addr(p.start_address()),
                Some(f.start_address())
            );
        }

        unmap_range(pages);

        for p in pages {
            assert_eq!(mapper().translate_addr(p.start_address()), None);
        }
    }

    fn map_frame_range_from_page_range_and_unmap() {
        let num = kernel_mmap::for_testing().end - kernel_mmap::for_testing().start;
        let num = NumOfPages::new(num.try_into().unwrap());

        let frames = phys::frame_allocator().alloc(num).unwrap();

        let pages = kernel_mmap::for_testing();

        let allocated_pages = unsafe { map_frame_range_from_page_range(pages, frames) };

        assert_eq!(allocated_pages, pages);

        for (p, f) in pages.into_iter().zip(frames.into_iter()) {
            assert_eq!(
                mapper().translate_addr(p.start_address()),
                Some(f.start_address())
            );
        }

        unmap_range(pages);
    }

    fn map_frame_range_from_page_range_fail() {
        let num = kernel_mmap::for_testing().end - kernel_mmap::for_testing().start + 1;
        let num = NumOfPages::new(num.try_into().unwrap());

        let frames = phys::frame_allocator().alloc(num).unwrap();

        let pages = kernel_mmap::for_testing();

        assert!(unsafe { try_map_frame_range_from_page_range(pages, frames).is_none() });
    }
}
