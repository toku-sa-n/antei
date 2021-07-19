use core::fmt::{self, Write};

#[macro_export]
macro_rules! uefi_print{
    ($st:expr,$($arg:tt)*)=>{
        $crate::io::_print($st,format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! uefi_println {
    ($st:expr) => {
        $crate::uefi_print!($st,"\n");
    };
    ($st:expr,$($arg:tt)*)=>{
        $crate::uefi_print!($st,"{}\n",format_args!($($arg)*));
    }
}

#[doc(hidden)]
pub fn _print(st: &mut crate::SystemTable, args: fmt::Arguments<'_>) {
    let mut con_out = st.con_out();

    let _ = con_out.write_fmt(args);
}
