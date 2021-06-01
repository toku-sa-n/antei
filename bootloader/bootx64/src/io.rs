use core::fmt::{self, Write};

#[macro_export]
macro_rules! uefi_print{
    ($st:expr,$($arg:tt)*)=>{
        $crate::io::_print($st,format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! uefi_println {
    (_:expr) => {
        $crate::uefi_print!("\n");
    };
    ($st:expr,$($arg:tt)*)=>{
        $crate::uefi_print!($st,"{}\n",format_args!($($arg)*));
    }
}

#[doc(hidden)]
pub fn _print(st: &mut uefi_wrapper::SystemTable, args: fmt::Arguments) {
    let mut con_out = st.con_out();

    let r = con_out.write_fmt(args);
    r.expect("Failed to print a string on the console.");
}
