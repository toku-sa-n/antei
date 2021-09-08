// `qemu_println!` locks the serial port it uses. When a process switch occurs while sending a
// string to the serial port, the port is not unlocked. If another process tries to print something
// while the interrupts are disabled (e.g., for the debug of process switch), it fails to lock the
// port because it has been already locked by the preceding process, so a deadlock happens. To
// avoid this, `qemu_printlnk` firstly disables interrupts, then prints a string. We cannot
// override `qemu_println` because of the name resolution.
macro_rules! qemu_printlnk {
    ($($arg:tt)*) => {
        $crate::interrupt::disable_interrupts_and_do(||{
            qemu_print::qemu_println!($($arg)*);
        });
    };
}
