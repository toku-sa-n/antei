use {config::MAX_PID, context::Context, core::cell::UnsafeCell, vm::Kbox, x86_64::VirtAddr};

pub(crate) use {manager::switch, pid::Pid};

mod context;
mod manager;

const LEAST_PRIORITY_LEVEL: usize = 1;
const GUARD_PAGE_SIZE: usize = 4096;
const KERNEL_STACK_BYTES: usize = 12288;

pub(super) fn init() {
    manager::init();

    manager::add_idle();
}

pub(super) struct Process {
    pid: Pid,
    context: UnsafeCell<Context>,
    priority: Priority,
    kernel_stack: Kbox<UnsafeCell<[u8; KERNEL_STACK_BYTES]>>,
    state: State,
}
impl Process {
    const KERNEL_STACK_MAGIC: [u8; 8] = [0x73, 0x74, 0x6b, 0x67, 0x75, 0x61, 0x72, 0x64];

    fn idle() -> Self {
        Self {
            pid: Pid::new(0),
            context: UnsafeCell::default(),
            priority: Priority::new(LEAST_PRIORITY_LEVEL),
            kernel_stack: Self::generate_kernel_stack(),
            state: State::Running,
        }
    }

    fn check_kernel_stack_guard(&self) {
        // SAFETY: The borrow checker ensures that there is no mutable references to the kernel
        // stack.
        let stack = unsafe { &*self.kernel_stack.get() };

        let magic = &stack[GUARD_PAGE_SIZE..GUARD_PAGE_SIZE + Self::KERNEL_STACK_MAGIC.len()];

        if magic != Self::KERNEL_STACK_MAGIC {
            panic!("The kernel stack is smashed.");
        }
    }

    fn kernel_stack_bottom_addr(&self) -> VirtAddr {
        let ptr = self.kernel_stack.get();

        // SAFETY: No references point to `kernel_stack`.
        VirtAddr::from_ptr(ptr) + unsafe { (&*ptr).len() }
    }

    fn generate_kernel_stack() -> Kbox<UnsafeCell<[u8; KERNEL_STACK_BYTES]>> {
        let mut stack = Kbox::new(UnsafeCell::new([0; KERNEL_STACK_BYTES]));

        for (i, c) in Self::KERNEL_STACK_MAGIC.iter().enumerate() {
            stack.get_mut()[GUARD_PAGE_SIZE + i] = *c;
        }

        stack
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum State {
    Running,
    Runnable,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Priority(usize);
impl Priority {
    fn new(priority: usize) -> Self {
        assert!(priority <= LEAST_PRIORITY_LEVEL, "Invalid priority.");

        Self(priority)
    }

    fn as_usize(self) -> usize {
        self.0
    }
}
