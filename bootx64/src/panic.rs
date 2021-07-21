use {core::panic::PanicInfo, qemu_print::qemu_println};

#[macro_export]
macro_rules! uefi_panic {
    ($st:expr) => {
        $crate::uefi_panic!($st, "explicit panic");
    };
    ($st:expr,$($t:tt)+)=>{
        $crate::uefi_println!($st,"panicked at '{}', {}:{}:{}",core::format_args!($($t)+),file!(),line!(),column!());
        panic!();
    }
}

#[panic_handler]
fn panic(i: &PanicInfo<'_>) -> ! {
    qemu_println!("{}", i);

    fini();
}

#[cfg(test_on_qemu)]
fn fini() -> ! {
    qemu::exit_failure();
}

#[cfg(not(test_on_qemu))]
fn fini() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
