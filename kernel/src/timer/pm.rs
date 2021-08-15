use {
    super::GenericAddressStructure,
    acpi::fadt::PmTimer,
    core::convert::TryInto,
    vm::accessor::{
        single::{read_only, ReadOnly},
        Mapper,
    },
    x86_64::{instructions::port::PortReadOnly, PhysAddr},
};

pub(super) enum RegisterReader {
    Memory(ReadOnly<u32>),
    Io(PortReadOnly<u32>),
}
impl RegisterReader {
    pub(super) fn new(timer: &PmTimer) -> Self {
        match timer.address() {
            // SAFETY: `addr` is taken from FADT.
            GenericAddressStructure::SystemMemory(addr) => unsafe { Self::Memory(read_only(addr)) },
            GenericAddressStructure::SystemIo(port) => {
                Self::Io(PortReadOnly::new(port.try_into().unwrap()))
            }
        }
    }

    pub(super) fn read(&mut self) -> u32 {
        match self {
            Self::Memory(reader) => reader.read_volatile(),
            // SAFETY: `Self::new` ensures that `reader` is the correct port reader.
            Self::Io(reader) => unsafe { reader.read() },
        }
    }
}

/// # Safety
///
/// `rsdp` must be the correct address of RSDP.
pub(super) unsafe fn timer_info_from_rsdp_addr(rsdp: PhysAddr) -> PmTimer {
    // SAFETY: The caller must ensure that `rsdp` is the correct address of RSDP.
    let tables = unsafe { acpi::Tables::from_rsdp_addr(rsdp, &Mapper) };
    let tables = tables.expect("Failed to get the information of ACPI.");

    let fadt = tables.fadt;
    let fadt = fadt.expect("No FADT information.");

    let timer_info = fadt.pm_timer();
    let timer_info = timer_info.expect("Failed to get the information of the ACPI PM Timer");
    timer_info.expect("ACPI PM Timer is not supported.")
}
