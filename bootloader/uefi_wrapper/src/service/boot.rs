use core::fmt;
use r_efi::efi;

pub struct Boot<'a>(&'a mut efi::BootServices);
impl<'a> Boot<'a> {
    pub(crate) fn new(bs: &'a mut efi::BootServices) -> Self {
        Self(bs)
    }
}
impl fmt::Debug for Boot<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Boot").finish()
    }
}
