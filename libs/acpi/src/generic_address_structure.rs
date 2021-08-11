use {
    crate::Error,
    core::convert::TryFrom,
    r_acpi::{SYSTEM_IO_SPACE, SYSTEM_MEMORY_SPACE},
    x86_64::PhysAddr,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GenericAddressStructure {
    SystemMemory(PhysAddr),
    SystemIo(u64),
}
impl TryFrom<r_acpi::GenericAddressStructure> for GenericAddressStructure {
    type Error = Error;

    fn try_from(value: r_acpi::GenericAddressStructure) -> Result<Self, Self::Error> {
        match value.address_space_id {
            SYSTEM_MEMORY_SPACE => Ok(Self::SystemMemory(PhysAddr::new(value.address))),
            SYSTEM_IO_SPACE => Ok(Self::SystemIo(value.address)),
            e => Err(Error::UnsupportedAddressSpaceId(e)),
        }
    }
}
