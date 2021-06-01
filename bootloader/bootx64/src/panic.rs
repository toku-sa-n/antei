use core::panic::PanicInfo;

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
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
