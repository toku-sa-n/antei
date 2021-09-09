use {
    crate::{
        gdt,
        process::{ipc, Pid},
    },
    aligned_ptr::ptr,
    core::convert::TryInto,
    ipc_api::syscalls::Ty,
    num_traits::FromPrimitive,
    posix::sys::types::Pid as PosixPid,
    x86_64::{
        registers::{
            control::{Efer, EferFlags},
            model_specific::{LStar, Msr, Star},
            rflags::RFlags,
        },
        VirtAddr,
    },
};

const IA32_FMASK: Msr = Msr::new(0xc000_0084);

pub(super) fn init() {
    register_handler();

    register_segments_with_star();

    // SAFETY: `register_handler` registers a system call handler and `register_segments_with_start`
    // registers segment selectors.
    unsafe {
        enable_syscall_and_sysret();
    }

    disable_interrupts_on_syscall();
}

#[no_mangle]
fn handle_syscall(index: u64, a1: u64, a2: u64) {
    match FromPrimitive::from_u64(index) {
        Some(Ty::Send) => {
            let to = Pid::new(a1 as _);
            let message = unsafe { ptr::get(a2 as *const _) };

            ipc::send(to, message);
        }
        Some(Ty::Receive) => {
            let from = if (a1 as PosixPid) < 0 {
                ipc::ReceiveFrom::Any
            } else {
                ipc::ReceiveFrom::Pid(Pid::new(a1 as _))
            };

            ipc::receive(from, a2 as *mut _);
        }
        None => panic!("Invalid system call index."),
    }
}

/// # Safety
///
/// The caller must ensure that the correct system call handler is registered with the LSTAR
/// register and segment selectors with STAR.
unsafe fn enable_syscall_and_sysret() {
    // SAFETY: The caller must register proper system call handler and segment selectors.
    unsafe {
        Efer::update(|efer| efer.insert(EferFlags::SYSTEM_CALL_EXTENSIONS));
    }
}

fn register_handler() {
    extern "sysv64" {
        fn asm_handle_syscall();
    }

    LStar::write(VirtAddr::new(
        (asm_handle_syscall as usize).try_into().unwrap(),
    ));
}

fn register_segments_with_star() {
    let r = Star::write(
        gdt::user_code_selector(),
        gdt::user_data_selector(),
        gdt::kernel_code_selector(),
        gdt::kernel_data_selector(),
    );
    r.expect("Failed to register segment registers with STAR.");
}

fn disable_interrupts_on_syscall() {
    // SAFETY: Disabling interrupts on a system call does not violate memory safety.
    unsafe {
        update_ia32_fmask(|mask| mask.insert(RFlags::INTERRUPT_FLAG));
    }
}

/// # Safety
///
/// See: [`x86_64::registers::rflags::write`].
unsafe fn update_ia32_fmask(f: impl FnOnce(&mut RFlags)) {
    let mut mask = read_ia32_fmask();

    f(&mut mask);

    // SAFETY: The caller must uphold the safety requirements.
    unsafe {
        write_ia32_fmask(mask);
    }
}

fn read_ia32_fmask() -> RFlags {
    // SAFETY: Reading from IA32_FMASK does not violate memory safety.
    let mask = unsafe { IA32_FMASK.read() };
    let mask = RFlags::from_bits(mask);
    mask.expect("Invalid rflags.")
}

/// # Safety
///
/// See [`x86_64::registers::rflag::write`].
unsafe fn write_ia32_fmask(mask: RFlags) {
    // SAFETY: The caller must uphold the safety requirements.
    unsafe {
        let mut reg = IA32_FMASK;

        reg.write(mask.bits());
    }
}
