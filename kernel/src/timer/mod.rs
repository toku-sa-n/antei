use {acpi::GenericAddressStructure, x86_64::PhysAddr};

mod apic;
mod pm;

/// # Safety
///
/// - `rsdp` must be the correct address of RSDP.
/// - The start address of the Local APIC registers must be `0xfee0_0000` (the default one).
pub(super) unsafe fn init(rsdp: PhysAddr) {
    // SAFETY: The caller must uphold the safety requirements.
    unsafe {
        apic::init(rsdp);
    }
}
