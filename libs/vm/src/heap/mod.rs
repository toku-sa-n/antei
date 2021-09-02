use {
    super::map,
    crate::NumOfPages,
    conquer_once::spin::Lazy,
    core::{
        alloc::{GlobalAlloc, Layout},
        convert::TryInto,
        ptr::{self, NonNull},
    },
    spinning_top::Spinlock,
    x86_64::structures::paging::PageTableFlags,
};

static HEAP: Heap = Heap(Lazy::new(|| {
    Spinlock::new(linked_list_allocator::Heap::empty())
}));

/// # Panics
///
/// This method panics if `layout.size() == 0`.
#[must_use]
pub fn alloc(layout: Layout) -> *mut u8 {
    assert_ne!(layout.size(), 0, "Size is 0.");

    // SAFETY: The size is not 0.
    unsafe { HEAP.alloc(layout) }
}

/// # Safety
///
/// See [`core::alloc::GlobalAlloc::dealloc`].
pub unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
    unsafe {
        HEAP.dealloc(ptr, layout);
    }
}

pub(super) fn init() {
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    let page_range = predefined_mmap::heap();

    unsafe {
        map::map_page_range_to_unused_frame_range(page_range, flags);
    }

    let start = page_range.start.start_address().as_u64();
    let start: usize = start.try_into().unwrap();

    unsafe {
        HEAP.0.lock().init(start, size().as_usize());
    }
}

struct Heap(Lazy<Spinlock<linked_list_allocator::Heap>>);
unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
            .lock()
            .allocate_first_fit(layout)
            .map_or(ptr::null_mut(), NonNull::as_ptr)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            self.0.lock().deallocate(NonNull::new(ptr).unwrap(), layout);
        }
    }
}

fn size() -> NumOfPages {
    let heap = predefined_mmap::heap();

    NumOfPages::new((heap.end - heap.start).try_into().unwrap())
}
