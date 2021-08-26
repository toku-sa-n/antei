use {
    crate::gdt::{user_code, user_data},
    address_space::AddressSpace,
    aligned_ptr::slice,
    context::Context,
    core::{
        convert::TryInto,
        sync::atomic::{AtomicUsize, Ordering},
    },
    heapless::Deque,
    os_units::NumOfPages,
    spinning_top::{const_spinlock, Spinlock},
    vm::accessor::single::write_only,
    x86_64::{
        registers::control::Cr3,
        structures::paging::{
            page::PageRange, FrameAllocator, Page, PageSize, PageTableFlags, PhysFrame, Size4KiB,
        },
        VirtAddr,
    },
};

mod address_space;
mod context;
mod elf;

const MAX_PROCESS: usize = 8;
const INIT_PROCESS_NAME: &str = "init";
const NULL_PROCESS: Option<Process> = None;
const STACK_END: VirtAddr = VirtAddr::new_truncate(0x8000_0000_0000_0000);
const STACK_START: VirtAddr = VirtAddr::new_truncate(STACK_END.as_u64() - 4 * Size4KiB::SIZE);

static PROCESS: [Option<Process>; MAX_PROCESS] = [NULL_PROCESS; MAX_PROCESS];
static ACTIVE_PIDS: Spinlock<Deque<Pid, MAX_PROCESS>> = const_spinlock(Deque::new());
static CURRENT_PID: Pid = Pid(1);

struct Process {
    pid: Pid,
    context: Context,
}
impl Process {
    unsafe fn load_init_proc() -> Option<Self> {
        let mut frame_allocator = vm::frame_allocator();

        let binary = cpio_reader::iter_files(initrd_binary())
            .find(|entry| entry.name() == INIT_PROCESS_NAME)?;
        let binary = binary.file();

        let instruction_pointer = unsafe { vm::map_elf(binary) };

        let pid = Pid::new()?;
        let pml4 = Self::generate_pml4_and_switch(&mut *frame_allocator)?;

        unsafe { Self::init_stack() };

        Some(Self {
            pid,
            context: Context {
                rsp: STACK_END.as_u64(),
                cr3: pml4.start_address().as_u64(),
                cs: user_code().0.into(),
                ss: user_data().0.into(),
                fs: user_data().0.into(),
                gs: user_data().0.into(),
                rip: instruction_pointer.as_u64(),
                rflags: 0x202,
                ..Context::default()
            },
        })
    }

    fn generate_pml4_and_switch(
        frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    ) -> Option<PhysFrame> {
        Self::generate_pml4(frame_allocator).map(|pml4| {
            Self::switch_to_pml4(pml4);

            pml4
        })
    }

    fn generate_pml4(frame_allocator: &mut impl FrameAllocator<Size4KiB>) -> Option<PhysFrame> {
        frame_allocator.allocate_frame().map(|frame| unsafe {
            write_only(frame.start_address()).write_volatile(vm::copy_current_pml4());
            frame
        })
    }

    fn switch_to_pml4(pml4: PhysFrame) {
        let (_, flags) = Cr3::read();

        unsafe {
            Cr3::write(pml4, flags);
        }
    }

    /// # Safety
    ///
    /// Call this function after switching to the newly created PML4.
    unsafe fn init_stack() {
        let flags =
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;

        unsafe {
            vm::map_page_range_to_unused_frame_range(Self::stack_range(), flags);
        }
    }

    fn stack_range() -> PageRange {
        let start = Page::containing_address(STACK_START);
        let end = Page::containing_address(STACK_END);

        PageRange { start, end }
    }
}

struct Pid(usize);
impl Pid {
    fn new() -> Option<Self> {
        static NEXT_PID_CANDIDATE: AtomicUsize = AtomicUsize::new(1);

        for _ in 0..MAX_PROCESS {
            let candidate = NEXT_PID_CANDIDATE.fetch_add(1, Ordering::Relaxed) % MAX_PROCESS;

            if PROCESS[candidate].is_none() {
                return Some(Self(candidate));
            }
        }

        None
    }
}

fn initrd_binary<'a>() -> &'a [u8] {
    use predefined_mmap::initrd;

    let num_of_pages = initrd().end - initrd().start;
    let num_of_pages = NumOfPages::<Size4KiB>::new(num_of_pages.try_into().unwrap());

    let start = initrd().start.start_address().as_ptr();

    unsafe { slice::from_raw_parts(start, num_of_pages.as_bytes().as_usize()) }
}
