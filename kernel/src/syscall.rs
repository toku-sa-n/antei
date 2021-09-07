use {
    crate::gdt,
    core::convert::TryInto,
    x86_64::{
        registers::{
            control::{Efer, EferFlags},
            model_specific::{LStar, Star},
        },
        VirtAddr,
    },
};

pub(super) fn init() {
    register_handler();

    register_segments_with_star();

    // SAFETY: `register_handler` registers a system call handler and `register_segments_with_start`
    // registers segment selectors.
    unsafe {
        enable_syscall_and_sysret();
    }
}

/// # Safety
///
/// The caller must ensure that the correct system call handler is registered with the LSTAR
/// register and segment selectors with STAR.
unsafe fn enable_syscall_and_sysret() {
    // SAFETY: Enabling `syscall` and `sysret` does not violate memory safety.
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
