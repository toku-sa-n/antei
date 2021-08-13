use {apic::local::EOI, kernel_mem::accessor::single::write_only, log::info};

#[no_mangle]
fn interrupt_handler_0x20() {
    // SAFETY: This OS does not change the start address of the Local APIC registers, thus `EOI` is
    // the correct address.
    unsafe {
        write_only(EOI).write_volatile(0_u32);
    };

    info!("Timer");
}
