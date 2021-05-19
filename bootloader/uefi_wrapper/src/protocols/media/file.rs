use r_efi::efi::protocols::file;

#[repr(transparent)]
pub struct File(file::Protocol);
