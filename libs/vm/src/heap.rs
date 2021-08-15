use {
    super::{map, phys},
    crate::NumOfPages,
    conquer_once::spin::Lazy,
    core::{
        alloc::{GlobalAlloc, Layout},
        convert::TryInto,
        ptr::{self, NonNull},
    },
    spinning_top::{Spinlock, SpinlockGuard},
};

static HEAP: Heap = Heap(Lazy::new(|| {
    Spinlock::new(linked_list_allocator::Heap::empty())
}));

pub fn alloc(layout: Layout) -> *mut u8 {
    assert_ne!(layout.size(), 0, "Size is 0.");

    // SAFETY: The size is not 0.
    unsafe { HEAP.alloc(layout) }
}

pub unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
    unsafe {
        HEAP.dealloc(ptr, layout);
    }
}

pub(super) fn init() {
    let frames = phys::frame_allocator().alloc(size());
    let frames = frames.expect("Failed to initialize heap.");

    let start = frames.start.start_address();
    let bytes = size().as_bytes();

    unsafe { map(start, bytes) };

    unsafe {
        HEAP.0
            .lock()
            .init(start.as_u64().try_into().unwrap(), bytes.as_usize());
    }
}

struct Heap(Lazy<Spinlock<linked_list_allocator::Heap>>);
unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
            .lock()
            .allocate_first_fit(layout)
            .map_or(ptr::null_mut(), |p| p.as_ptr())
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
