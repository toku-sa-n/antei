use {acpi::GenericAddressStructure, log::info, x86_64::PhysAddr};

mod apic;
mod pm;

/// # Safety
///
/// - `rsdp` must be the correct address of RSDP.
/// - The start address of the Local APIC registers must be `0xfee0_0000` (the default one).
pub(super) unsafe fn init(rsdp: PhysAddr) {
    // SAFETY: The caller must ensure that `rsdp` is the correct address of RSDP.
    let measurer = unsafe { apic::FrequencyMeasurer::from_rsdp_addr(rsdp) };

    // SAFETY: The caller must not change the start adress of the Local APIC registers.
    info!("The frequency of the Local APIC Timer is {}MHz.", unsafe {
        measurer.measure()
    });
}
