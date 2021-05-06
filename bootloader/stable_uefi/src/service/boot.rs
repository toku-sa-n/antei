use r_efi::efi;

#[repr(transparent)]
pub struct Boot(efi::BootServices);
