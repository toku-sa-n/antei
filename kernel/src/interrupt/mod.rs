mod handler;
pub(super) mod idt;

use x86_64::instructions::interrupts;

pub(crate) fn disable_interrupts_and_do<T>(f: impl FnOnce() -> T) -> T {
    let interrupts_enabled = interrupts::are_enabled();

    if interrupts_enabled {
        interrupts::disable();
    }

    let r = f();

    if interrupts_enabled {
        interrupts::enable();
    }

    r
}
