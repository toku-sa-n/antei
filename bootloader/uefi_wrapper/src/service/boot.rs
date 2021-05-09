use crate::system_table::SystemTable;
use core::fmt;
use r_efi::efi;

pub struct Boot<'a>(&'a mut efi::BootServices, &'a mut SystemTable);
impl<'a> From<&'a mut SystemTable> for Boot<'a> {
    fn from(s: &'a mut SystemTable) -> Self {
        let s_ptr = s.get_ptr();

        // SAFETY: `SystemTable` is created only from the argument of `efi_main`. We must trust the
        // argument is a valid pointer.
        //
        // There exists only one `SystemTable`, so do `Boot`. This is why the mutable reference is
        // only one it exists.
        let bs = unsafe { &mut *(*s_ptr).boot_services };

        Self(bs, s)
    }
}
impl fmt::Debug for Boot<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Boot").finish()
    }
}
