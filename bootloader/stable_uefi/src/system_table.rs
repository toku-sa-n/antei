use r_efi::efi;

#[repr(transparent)]
pub struct SystemTable(efi::SystemTable);
