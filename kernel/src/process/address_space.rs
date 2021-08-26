use {
    core::convert::TryFrom,
    vm::{
        accessor::single::{read_write, ReadWrite},
        copy_current_pml4,
    },
    x86_64::structures::paging::{
        mapper::{
            FlagUpdateError, MapToError, MapperFlush, MapperFlushAll, TranslateError, UnmapError,
        },
        page::PageRange,
        page_table::{FrameError, PageTableEntry},
        FrameAllocator, Mapper, Page, PageSize, PageTable, PageTableFlags, PageTableIndex,
        PhysFrame, Size4KiB,
    },
};

pub(super) struct AddressSpace {
    pml4: PhysFrame,
}
impl AddressSpace {
    pub(super) fn new(frame_allocator: &mut impl FrameAllocator<Size4KiB>) -> Option<Self> {
        frame_allocator.allocate_frame().map(|pml4| {
            // SAFETY: No one points to `pml4`.
            unsafe {
                init_pml4(pml4);
            }

            Self { pml4 }
        })
    }

    pub(super) fn pml4(&self) -> PhysFrame {
        self.pml4
    }

    pub(super) unsafe fn map_range_to_unused(
        &mut self,
        range: PageRange,
        flags: PageTableFlags,
        frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    ) -> Result<(), MapToError<Size4KiB>> {
        for page in range {
            unsafe {
                self.map_to_unused(page, flags, frame_allocator)?.flush();
            }
        }

        Ok(())
    }

    pub(super) unsafe fn map_to_unused(
        &mut self,
        page: Page,
        flags: PageTableFlags,
        frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>> {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;

        unsafe { self.map_to(page, frame, flags, frame_allocator) }
    }

    pub(super) unsafe fn update_flags_for_range(
        &mut self,
        range: PageRange,
        flags: PageTableFlags,
    ) -> Result<(), FlagUpdateError> {
        for page in range {
            unsafe {
                self.update_flags(page, flags)?.flush();
            }
        }

        Ok(())
    }

    fn set_flags_of(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
        table: PageTableTypes,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        let handler = |_: &mut _| Err(PageTableWalkerError::FrameNotPresent);

        let mut table_ptr = unsafe {
            PageTableWalker::new(self.pml4, handler, page, table)
                .walk_down()
                .map_err(|e| FlagUpdateError::try_from(e).unwrap())?
        };

        let entry = &table_ptr.read_page_table()[page.p4_index()];

        if entry.is_unused() {
            Err(FlagUpdateError::PageNotMapped)
        } else {
            table_ptr.update_page_table(|table| {
                table[page.p4_index()].set_flags(flags);
            });

            Ok(MapperFlushAll::new())
        }
    }
}
impl Mapper<Size4KiB> for AddressSpace {
    unsafe fn map_to_with_table_flags<A>(
        &mut self,
        page: Page<Size4KiB>,
        frame: PhysFrame<Size4KiB>,
        flags: PageTableFlags,
        parent_table_flags: PageTableFlags,
        frame_allocator: &mut A,
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>>
    where
        Self: Sized,
        A: FrameAllocator<Size4KiB> + ?Sized,
    {
        let create_next_table = |entry: &mut PageTableEntry| {
            let _ = &frame_allocator;

            let next_page_table = frame_allocator
                .allocate_frame()
                .ok_or(PageTableWalkerError::FrameAllocationFailed)?;

            entry.set_frame(next_page_table, parent_table_flags);

            Ok(())
        };

        let mut pt_ptr = unsafe {
            PageTableWalker::new(self.pml4, create_next_table, page, PageTableTypes::Pt)
                .walk_down()
                .map_err(|e| MapToError::try_from(e).unwrap())?
        };

        let pt_entry = &pt_ptr.read_page_table()[page.p1_index()];

        if pt_entry.is_unused() {
            pt_ptr.update_page_table(|table| {
                table[page.p1_index()].set_frame(frame, flags);
            });

            Ok(MapperFlush::new(page))
        } else {
            Err(MapToError::PageAlreadyMapped(pt_entry.frame().unwrap()))
        }
    }

    fn unmap(
        &mut self,
        page: Page<Size4KiB>,
    ) -> Result<(PhysFrame<Size4KiB>, MapperFlush<Size4KiB>), UnmapError> {
        let handler = |_: &mut PageTableEntry| Err(PageTableWalkerError::FrameNotPresent);

        let mut pt = unsafe {
            PageTableWalker::new(self.pml4, handler, page, PageTableTypes::Pt)
                .walk_down()
                .map_err(|e| UnmapError::try_from(e).unwrap())?
        };

        let pt_entry = &pt.read_page_table()[page.p1_index()];

        let frame = pt_entry.frame().map_err(frame_error_to_unmap_error)?;

        pt.update_page_table(|table| table[page.p1_index()].set_unused());

        Ok((frame, MapperFlush::new(page)))
    }

    unsafe fn update_flags(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size4KiB>, FlagUpdateError> {
        self.set_flags_of(page, flags, PageTableTypes::Pt)
            .map(|_| MapperFlush::new(page))
    }

    unsafe fn set_flags_p4_entry(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        self.set_flags_of(page, flags, PageTableTypes::Pml4)
    }

    unsafe fn set_flags_p3_entry(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        self.set_flags_of(page, flags, PageTableTypes::Pdpt)
    }

    unsafe fn set_flags_p2_entry(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        self.set_flags_of(page, flags, PageTableTypes::Pd)
    }

    fn translate_page(&self, page: Page<Size4KiB>) -> Result<PhysFrame<Size4KiB>, TranslateError> {
        let handler = |_: &mut PageTableEntry| Err(PageTableWalkerError::FrameNotPresent);

        let pt = unsafe {
            PageTableWalker::new(self.pml4, handler, page, PageTableTypes::Pt)
                .walk_down()
                .map_err(|e| TranslateError::try_from(e).unwrap())?
        };

        pt.read_page_table()[page.p1_index()]
            .frame()
            .map_err(frame_error_to_translate_error)
    }
}

#[derive(Debug)]
struct PageTablePointer(ReadWrite<PageTable>);
impl PageTablePointer {
    /// # Safety
    ///
    /// - `frame` must be a correct page table.
    /// - Any references must not point to one of the page tables accessible by tracing from page
    /// table `frame`.
    unsafe fn new(frame: PhysFrame) -> Self {
        // SAFETY: The caller must uphold the safety requirements.
        Self(unsafe { read_write(frame.start_address()) })
    }

    fn read_page_table(&self) -> PageTable {
        self.0.read_volatile()
    }

    fn update_page_table(&mut self, f: impl FnOnce(&mut PageTable)) {
        self.0.update_volatile(f);
    }

    fn jump_to_sub_table(self, i: PageTableIndex) -> Result<Self, FrameError> {
        match self.read_page_table()[i].frame() {
            Ok(frame) => {
                // This drop is necessary because, without it, two mutable references to the same
                // page table may be created if the entry is the recursive entry.
                drop(self);

                // SAFETY: `frame` is a page table and the caller ensures that no references point
                // to `frame`.
                Ok(unsafe { Self::new(frame) })
            }
            Err(e) => Err(e),
        }
    }
}

#[must_use = "Call `walk_down`."]
struct PageTableWalker<T>
where
    T: FnMut(&mut PageTableEntry) -> Result<(), PageTableWalkerError>,
{
    current: PageTablePointer,
    no_next_table_handler: T,
    page: Page,
    walk_down_to: PageTableTypes,
}
impl<T> PageTableWalker<T>
where
    T: FnMut(&mut PageTableEntry) -> Result<(), PageTableWalkerError>,
{
    /// # Safety
    ///
    /// - `pml4` must be a frame that contains a correct page table.
    /// - Any references must not point to one of the page tables accessible by tracing page table
    /// `pml4` entries.
    /// - `no_next_table_handler` must handle an unused entry correctly. It must not set a wrong
    /// address to next page.
    unsafe fn new(
        pml4: PhysFrame,
        no_next_table_handler: T,
        page: Page,
        walk_down_to: PageTableTypes,
    ) -> Self {
        Self {
            current: unsafe { PageTablePointer::new(pml4) },
            no_next_table_handler,
            page,
            walk_down_to,
        }
    }

    fn walk_down(mut self) -> Result<PageTablePointer, PageTableWalkerError> {
        let indexes = [
            self.page.p4_index(),
            self.page.p3_index(),
            self.page.p2_index(),
        ];

        for &index in &indexes[0..self.walk_down_to as _] {
            let mut entry = &mut self.current.read_page_table()[index];

            if entry.frame().is_err() {
                (self.no_next_table_handler)(&mut entry)?;

                self.current
                    .update_page_table(|table| table[index] = entry.clone());
            }

            self.current = self
                .current
                .jump_to_sub_table(index)
                .map_err(PageTableWalkerError::from)?;
        }

        Ok(self.current)
    }
}

#[derive(Copy, Clone, Debug)]
enum PageTableWalkerError {
    FrameAllocationFailed,
    HugeFrame,
    FrameNotPresent,
}
impl From<FrameError> for PageTableWalkerError {
    fn from(e: FrameError) -> Self {
        match e {
            FrameError::FrameNotPresent => Self::FrameNotPresent,
            FrameError::HugeFrame => Self::HugeFrame,
        }
    }
}
impl<S: PageSize> TryFrom<PageTableWalkerError> for MapToError<S> {
    type Error = PageTableWalkerError;

    fn try_from(e: PageTableWalkerError) -> Result<Self, Self::Error> {
        match e {
            PageTableWalkerError::FrameAllocationFailed => Ok(MapToError::FrameAllocationFailed),
            PageTableWalkerError::HugeFrame => Ok(MapToError::ParentEntryHugePage),
            PageTableWalkerError::FrameNotPresent => Err(e),
        }
    }
}
impl TryFrom<PageTableWalkerError> for UnmapError {
    type Error = PageTableWalkerError;

    fn try_from(e: PageTableWalkerError) -> Result<Self, Self::Error> {
        match e {
            PageTableWalkerError::FrameAllocationFailed => Err(e),
            PageTableWalkerError::HugeFrame => Ok(UnmapError::ParentEntryHugePage),
            PageTableWalkerError::FrameNotPresent => Ok(UnmapError::PageNotMapped),
        }
    }
}
impl TryFrom<PageTableWalkerError> for FlagUpdateError {
    type Error = PageTableWalkerError;

    fn try_from(e: PageTableWalkerError) -> Result<Self, Self::Error> {
        match e {
            PageTableWalkerError::FrameAllocationFailed => Err(e),
            PageTableWalkerError::HugeFrame => Ok(FlagUpdateError::ParentEntryHugePage),
            PageTableWalkerError::FrameNotPresent => Ok(FlagUpdateError::PageNotMapped),
        }
    }
}
impl TryFrom<PageTableWalkerError> for TranslateError {
    type Error = PageTableWalkerError;

    fn try_from(e: PageTableWalkerError) -> Result<Self, Self::Error> {
        match e {
            PageTableWalkerError::FrameAllocationFailed => Err(e),
            PageTableWalkerError::HugeFrame => Ok(TranslateError::ParentEntryHugePage),
            PageTableWalkerError::FrameNotPresent => Ok(TranslateError::PageNotMapped),
        }
    }
}

enum PageTableTypes {
    Pml4 = 4,
    Pdpt = 3,
    Pd = 2,
    Pt = 1,
}

fn frame_error_to_unmap_error(e: FrameError) -> UnmapError {
    match e {
        FrameError::FrameNotPresent => UnmapError::PageNotMapped,
        FrameError::HugeFrame => UnmapError::ParentEntryHugePage,
    }
}

fn frame_error_to_translate_error(e: FrameError) -> TranslateError {
    match e {
        FrameError::FrameNotPresent => TranslateError::PageNotMapped,
        FrameError::HugeFrame => TranslateError::ParentEntryHugePage,
    }
}

/// # Safety
///
/// Any references must not point to `pml4`.
unsafe fn init_pml4(pml4: PhysFrame) {
    use PageTableFlags as Flags;

    let pml4_addr = pml4.start_address();

    let mut accessor = unsafe { read_write::<PageTable>(pml4_addr) };

    accessor.update_volatile(|pml4| {
        let flags = Flags::PRESENT | Flags::WRITABLE | Flags::USER_ACCESSIBLE;

        pml4[0x510].set_addr(pml4_addr, flags);
        pml4[0x511] = copy_current_pml4()[0x511].clone();
    });
}
