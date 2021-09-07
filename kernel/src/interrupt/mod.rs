mod handler;
pub(super) mod idt;

use x86_64::instructions::interrupts;

pub(crate) fn disable_do_restore<T>(f: impl FnOnce() -> T) -> T {
    let interrupts_enabled = interrupts::are_enabled();

    if interrupts_enabled {
        log::info!("Disabling interrupts.");
        interrupts::disable();
    }

    let r = f();

    if interrupts_enabled {
        log::info!("Enabling interrupts.");
        interrupts::enable();
    }

    r
}
