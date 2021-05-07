use core::fmt;
use r_efi::efi;

#[repr(transparent)]
pub struct Handle(efi::Handle);
impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Handle").finish()
    }
}
