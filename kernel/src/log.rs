use {
    log::{set_logger, Level, LevelFilter, Log, Metadata, Record},
    qemu_print::qemu_println,
};

static LOGGER: Logger = Logger;

struct Logger;
impl Log for Logger {
    #[cfg(test_on_qemu)]
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= Level::Debug
    }

    #[cfg(not(test_on_qemu))]
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= Level::Info
    }

    #[cfg(test_on_qemu)]
    fn log(&self, record: &Record<'_>) {
        if self.enabled(record.metadata()) {
            if let (Some(file), Some(line)) = (record.file(), record.line()) {
                qemu_println!("[{}:{}] {} - {}", file, line, record.level(), record.args());
            } else {
                qemu_println!("{} - {}", record.level(), record.args());
            }
        }
    }

    #[cfg(not(test_on_qemu))]
    fn log(&self, record: &Record<'_>) {
        if self.enabled(record.metadata()) {
            qemu_println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub(super) fn init() {
    set_logger(&LOGGER).expect("Failed to initialize a logger.");

    #[cfg(test_on_qemu)]
    log::set_max_level(LevelFilter::Debug);

    #[cfg(not(test_on_qemu))]
    log::set_max_level(LevelFilter::Info);
}
