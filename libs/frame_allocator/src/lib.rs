#![cfg_attr(not(test), no_std)]

use {
    arrayvec::ArrayVec,
    core::{
        convert::{TryFrom, TryInto},
        fmt,
    },
    os_units::NumOfPages,
    uefi::service::boot::{MemoryDescriptor, CONVENTIONAL_MEMORY},
    x86_64::{
        structures::paging::{
            frame::PhysFrameRange, page::AddressNotAligned, FrameAllocator as FrameAllocatorTrait,
            FrameDeallocator, PageSize, PhysFrame,
        },
        PhysAddr,
    },
};

#[derive(PartialEq, Eq, Debug)]
pub struct FrameAllocator<S: PageSize, const N: usize>(ArrayVec<FrameDescriptor<S>, N>);
impl<S: PageSize, const N: usize> FrameAllocator<S, N> {
    #[must_use]
    pub fn new() -> Self {
        Self(ArrayVec::new_const())
    }

    pub fn init(&mut self, mmap: &[MemoryDescriptor]) {
        mmap.iter().filter(|d| is_conventional(d)).for_each(|d| {
            let _ = &self;
            self.init_for_descriptor(d)
        })
    }

    fn init_for_descriptor(&mut self, descriptor: &MemoryDescriptor) {
        let start = PhysAddr::new(descriptor.physical_start);
        assert!(
            start.is_aligned(S::SIZE),
            "The address is not page-aligned."
        );

        let num = NumOfPages::new(descriptor.number_of_pages.try_into().unwrap());

        let range = phys_frame_range_from_start_and_num(start, num);
        let range = range.expect("The address is not aligned.");

        let frames = FrameDescriptor::new_for_available(range);

        self.0.push(frames);
    }
}
impl<S: PageSize, const N: usize> FrameAllocator<S, N> {
    pub fn alloc(&mut self, n: NumOfPages<S>) -> Option<PhysFrameRange<S>> {
        (0..self.0.len()).find_map(|i| {
            self.0[i].is_available_for_allocating(n).then(|| {
                let _ = &self;
                self.alloc_from_frames_at(i, n)
            })
        })
    }

    fn alloc_from_frames_at(&mut self, i: usize, n: NumOfPages<S>) -> PhysFrameRange<S> {
        if self.0[i].is_splittable(n) {
            self.split_frames(i, n);
        }

        self.0[i].available = false;
        self.0[i].range
    }

    fn split_frames(&mut self, i: usize, num_of_pages: NumOfPages<S>) {
        assert!(self.0[i].available, "Frames are not available.");
        assert!(
            self.0[i].num_of_pages() > num_of_pages,
            "Insufficient number of frames."
        );

        self.split_frames_unchecked(i, num_of_pages)
    }

    fn split_frames_unchecked(&mut self, i: usize, requested: NumOfPages<S>) {
        let requested: u64 = requested.as_usize().try_into().unwrap();

        let new_frames_start = self.0[i].range.start + requested;
        let new_frames_range = PhysFrameRange {
            start: new_frames_start,
            end: self.0[i].range.end,
        };
        let new_frames = FrameDescriptor::new_for_available(new_frames_range);

        self.0[i].range.end = new_frames_start;
        self.0.insert(i + 1, new_frames);
    }
}
impl<S: PageSize, const N: usize> FrameAllocator<S, N> {
    pub fn dealloc(&mut self, first_frame: PhysFrame<S>) {
        for i in 0..self.0.len() {
            if self.0[i].range.start == first_frame && !self.0[i].available {
                return self.free_memory_for_frames_at(i);
            }
        }
    }

    fn free_memory_for_frames_at(&mut self, i: usize) {
        self.0[i].available = true;
        self.merge_before_and_after_frames(i);
    }

    fn merge_before_and_after_frames(&mut self, i: usize) {
        if self.mergeable_to_next_frames(i) {
            self.merge_to_next_frames(i);
        }

        if i > 0 && self.mergeable_to_next_frames(i - 1) {
            self.merge_to_next_frames(i - 1);
        }
    }

    fn mergeable_to_next_frames(&self, i: usize) -> bool {
        if i >= self.0.len() - 1 {
            return false;
        }

        let node = &self.0[i];
        let next = &self.0[i + 1];

        node.is_mergeable(next)
    }

    fn merge_to_next_frames(&mut self, i: usize) {
        let n = self.0[i + 1].num_of_pages();
        self.0[i].range.end += u64::try_from(n.as_usize()).unwrap();
        self.0.remove(i + 1);
    }
}
unsafe impl<S: PageSize, const N: usize> FrameAllocatorTrait<S> for FrameAllocator<S, N> {
    fn allocate_frame(&mut self) -> Option<PhysFrame<S>> {
        let frames = self.alloc(NumOfPages::new(1))?;

        assert_eq!(frames.start + 1, frames.end, "Too many frames.");

        Some(frames.start)
    }
}
impl<S: PageSize, const N: usize> FrameDeallocator<S> for FrameAllocator<S, N> {
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame<S>) {
        self.dealloc(frame);
    }
}
impl<S: PageSize, const N: usize> Default for FrameAllocator<S, N> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(PartialEq, Eq)]
struct FrameDescriptor<S: PageSize> {
    range: PhysFrameRange<S>,
    available: bool,
}
impl<S: PageSize> FrameDescriptor<S> {
    fn new_for_available(range: PhysFrameRange<S>) -> Self {
        Self {
            range,
            available: true,
        }
    }

    #[cfg(test)]
    fn new_for_used(range: PhysFrameRange<S>) -> Self {
        Self {
            range,
            available: false,
        }
    }

    fn is_splittable(&self, requested: NumOfPages<S>) -> bool {
        self.num_of_pages() > requested
    }

    fn is_available_for_allocating(&self, request_num_of_pages: NumOfPages<S>) -> bool {
        self.num_of_pages() >= request_num_of_pages && self.available
    }

    fn is_mergeable(&self, other: &Self) -> bool {
        self.available && other.available && self.is_consecutive(other)
    }

    fn is_consecutive(&self, other: &Self) -> bool {
        self.range.end == other.range.start
    }

    fn num_of_pages(&self) -> NumOfPages<S> {
        NumOfPages::new((self.range.end - self.range.start).try_into().unwrap())
    }
}
impl<S: PageSize> fmt::Debug for FrameDescriptor<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suffix = if self.available { "Available" } else { "Used" };
        write!(
            f,
            "Frames::<{}>({:?} .. {:?})",
            suffix, self.range.start, self.range.end,
        )
    }
}

fn is_conventional(d: &MemoryDescriptor) -> bool {
    d.r#type == CONVENTIONAL_MEMORY
}

fn phys_frame_range_from_start_and_num<S: PageSize>(
    p: PhysAddr,
    n: NumOfPages<S>,
) -> Result<PhysFrameRange<S>, AddressNotAligned> {
    let start = PhysFrame::from_start_address(p)?;

    let end = start + u64::try_from(n.as_usize()).unwrap();

    Ok(PhysFrameRange { start, end })
}

#[cfg(test)]
mod tests {
    use {
        super::{phys_frame_range_from_start_and_num, FrameAllocator, FrameDescriptor},
        os_units::NumOfPages,
        x86_64::{
            structures::paging::{frame::PhysFrameRange, PageSize, PhysFrame, Size4KiB},
            PhysAddr,
        },
    };

    macro_rules! descriptor {
        (A $start:expr => $end:expr) => {
            FrameDescriptor::<Size4KiB>::new_for_available(
                phys_frame_range_from_start_and_num(
                    PhysAddr::new($start),
                    os_units::Bytes::new($end - $start).as_num_of_pages(),
                )
                .unwrap(),
            )
        };
        (U $start:expr => $end:expr) => {
            FrameDescriptor::new_for_used(
                phys_frame_range_from_start_and_num(
                    PhysAddr::new($start),
                    os_units::Bytes::new($end - $start).as_num_of_pages(),
                )
                .unwrap(),
            )
        };
    }

    macro_rules! arrayvec {
        ($($elem:expr),*) => {
            {
                let mut v=arrayvec::ArrayVec::new();
                $(
                    v.push($elem);
                )*
                    v
            }
        };
    }

    macro_rules! allocator {
        ($($is_available:ident $start:expr => $end:expr),*$(,)*) => {
            FrameAllocator::<_,8>(arrayvec![
                $(descriptor!($is_available $start => $end)),*
            ]
            )
        };
    }

    macro_rules! phys_frame_range {
        ($start:expr => $end:expr) => {
            PhysFrameRange {
                start: PhysFrame::from_start_address(PhysAddr::new($start)).unwrap(),
                end: PhysFrame::from_start_address(PhysAddr::new($end)).unwrap(),
            }
        };
    }

    #[test]
    fn fail_to_allocate() {
        let mut f = allocator!(
            A 0 => 0x1000,
            A 0x2000 => 0xc000,
            U 0xc000 => 0x10000,
            U 0x10000 => 0x13000,
            A 0x13000 => 0x15000,
        );

        let a = f.alloc(NumOfPages::new(200));
        assert!(a.is_none());
    }

    #[test]
    fn allocate_not_power_of_two() {
        let mut f = allocator!(
            A 0 => 0x1000,
            A 0x2000 => 0xc000,
            U 0xc000 => 0x10000,
        );

        let a = f.alloc(NumOfPages::new(3));

        assert_eq!(a, Some(phys_frame_range!(0x2000 => 0x5000)));
        assert_eq!(
            f,
            allocator!(
                A 0 => 0x1000,
                U 0x2000 => 0x5000,
                A 0x5000 => 0xc000,
                U 0xc000 => 0x10000,
            )
        )
    }

    #[test]
    fn allocate_full_frames() {
        let mut f = allocator!(A 0 => 0x3000);
        let a = f.alloc(NumOfPages::new(3));

        assert_eq!(a, Some(phys_frame_range!(0 => 0x3000)));
        assert_eq!(f, allocator!(U 0 => 0x3000));
    }

    #[test]
    fn free_single_frames() {
        let mut f = allocator!(U 0 => 0x3000);

        f.dealloc(frame(0));

        assert_eq!(f, allocator!(A 0 => 0x3000));
    }

    #[test]
    fn free_and_merge_with_before() {
        let mut f = allocator!(
            A 0 => 0x1000,
            A 0x2000 => 0xc000,
            U 0xc000 => 0x10000,
        );

        f.dealloc(frame(0xc000));

        assert_eq!(
            f,
            allocator! (
                A 0 => 0x1000,
                A 0x2000 => 0x10000
            )
        )
    }

    #[test]
    fn free_and_merge_with_after() {
        let mut f = allocator!(
            U 0 => 0x3000,
            A 0x3000 => 0x5000,
        );

        f.dealloc(frame(0));

        assert_eq!(
            f,
            allocator!(
                A 0 => 0x5000,
            )
        )
    }

    #[test]
    fn free_and_merge_with_before_and_after() {
        let mut f = allocator!(
            A 0 => 0x3000,
            U 0x3000 => 0x5000,
            A 0x5000 => 0x10000,
        );

        f.dealloc(frame(0x3000));

        assert_eq!(f, allocator!(A 0 => 0x10000))
    }

    #[test]
    fn mergable_two_frmaes() {
        let f1 = descriptor!(A 0x2000 => 0xc000);
        let f2 = descriptor!(A 0xc000 => 0x10000);

        assert!(f1.is_mergeable(&f2));
    }

    fn frame<S: PageSize>(start: u64) -> PhysFrame<S> {
        PhysFrame::from_start_address(PhysAddr::new(start)).unwrap()
    }
}
