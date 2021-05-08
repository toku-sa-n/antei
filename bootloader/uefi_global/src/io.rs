use core::fmt;
use core::fmt::Write;
use log::Log;

static LOGGER: Logger = Logger;

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

pub fn _print(args: fmt::Arguments<'_>) {
    let mut st = crate::system_table();
    let mut stdout = st.con_out();

    let r = stdout.write_fmt(args);
    r.expect("Failed to print a string.");
}

pub(super) fn init() {
    init_logger();
}

struct Logger;
impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

fn init_logger() {
    let r = log::set_logger(&LOGGER).map(|_| log::set_max_level(log::LevelFilter::Info));
    r.expect("Failed to initialize the global logger.");
}
