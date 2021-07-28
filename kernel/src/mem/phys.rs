use {
    conquer_once::spin::Lazy,
    frame_allocator::FrameAllocator,
    spinning_top::{Spinlock, SpinlockGuard},
    uefi_wrapper::service::boot::MemoryDescriptor,
    x86_64::structures::paging::Size4KiB,
};

static FRAME_ALLOCATOR: Lazy<Spinlock<FrameAllocator<Size4KiB>>> =
    Lazy::new(|| Spinlock::new(FrameAllocator::new()));

pub(super) fn init(mmap: &[MemoryDescriptor]) {
    frame_allocator().init(mmap);

    #[cfg(test_on_qemu)]
    tests::main();
}

pub(super) fn frame_allocator<'a>() -> SpinlockGuard<'a, FrameAllocator<Size4KiB>> {
    let f = FRAME_ALLOCATOR.try_lock();

    f.expect("Failed to acquire the lock of the frame allocator.")
}

#[cfg(test_on_qemu)]
mod tests {
    use {super::frame_allocator, crate::NumOfPages, x86_64::PhysAddr};

    pub(super) fn main() {
        allocate_single_page_and_dealloc();
    }

    fn allocate_single_page_and_dealloc() {
        let p = alloc(NumOfPages::new(1));
        let p = p.expect("Failed to allocate a page.");

        dealloc(p);
    }

    #[must_use]
    fn alloc(n: NumOfPages) -> Option<PhysAddr> {
        frame_allocator().alloc(n)
    }

    fn dealloc(a: PhysAddr) {
        frame_allocator().dealloc(a)
    }
}
