use {
    crate::NumOfPages,
    frame_allocator::FrameAllocator,
    spinning_top::{const_spinlock, Spinlock, SpinlockGuard},
    uefi_wrapper::service::boot::MemoryDescriptor,
    x86_64::PhysAddr,
};

static FRAME_ALLOCATOR: Spinlock<FrameAllocator> = const_spinlock(FrameAllocator::new());

pub fn init(mmap: &[MemoryDescriptor]) {
    frame_allocator().init(mmap);

    #[cfg(feature = "test_on_qemu")]
    tests::main();
}

pub fn alloc(n: NumOfPages) -> Option<PhysAddr> {
    frame_allocator().alloc(n)
}

pub fn dealloc(a: PhysAddr) {
    frame_allocator().dealloc(a)
}

fn frame_allocator<'a>() -> SpinlockGuard<'a, FrameAllocator> {
    let f = FRAME_ALLOCATOR.try_lock();

    f.expect("Failed to acquire the lock of the frame allocator.")
}

#[cfg(feature = "test_on_qemu")]
mod tests {
    use {
        super::{alloc, dealloc},
        crate::NumOfPages,
    };

    pub(super) fn main() {
        allocate_single_page();
    }

    fn allocate_single_page() {
        let p = alloc(NumOfPages::new(1));
        let p = p.expect("Failed to allocate a page.");

        dealloc(p);
    }
}
