use core::fmt;
use r_efi::efi;

pub struct Boot<'a>(pub(crate) &'a efi::BootServices);
impl<'a> fmt::Debug for Boot<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Boot").finish()
    }
}
