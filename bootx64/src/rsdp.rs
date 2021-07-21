use {
    crate::SystemTable,
    uefi_wrapper::system_table::{ConfigurationTable, EFI_ACPI_TABLE_GUID},
    x86_64::PhysAddr,
};

pub fn get(st: &SystemTable) -> PhysAddr {
    st.configuration_table()
        .iter()
        .find_map(try_get_from_configuration_table)
        .expect("This machine does not have RSDP.")
}

fn try_get_from_configuration_table(c: &ConfigurationTable) -> Option<PhysAddr> {
    (c.vendor_guid == EFI_ACPI_TABLE_GUID).then(|| PhysAddr::new(c.vendor_table as u64))
}
