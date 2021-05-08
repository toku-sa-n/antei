use core::fmt;
use core::fmt::Write;

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };
    ($($arg:tt)*)=>{
        $crate::print!("{}\n",core::format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::_print(core::format_args!($($arg)*));
    };
}

pub fn _print(args: fmt::Arguments) {
    let mut st = crate::system_table();
    let mut stdout = st.con_out();

    let r = stdout.write_fmt(args);
    r.expect("Failed to print a string.");
}
