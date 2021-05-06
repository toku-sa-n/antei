use r_efi::efi;

pub struct Boot<'a>(pub(crate) &'a efi::BootServices);
