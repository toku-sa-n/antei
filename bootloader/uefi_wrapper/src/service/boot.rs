use core::fmt;
use r_efi::efi;

pub struct Boot<'a>(pub(crate) &'a mut efi::BootServices);
impl fmt::Debug for Boot<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Boot").finish()
    }
}
