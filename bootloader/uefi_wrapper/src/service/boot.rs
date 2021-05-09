use crate::result;
use crate::result::Result;
use crate::system_table::SystemTable;
use core::ffi::c_void;
use core::fmt;
use core::mem::MaybeUninit;
use core::ptr;
use r_efi::efi;

pub struct Boot<'a>(&'a mut efi::BootServices, &'a mut SystemTable);
impl<'a> Boot<'a> {
    pub fn locate_protocol_without_registration(
        &mut self,
        mut guid: efi::Guid,
    ) -> Result<*mut c_void> {
        let mut protocol = MaybeUninit::uninit();
        let s = (self.0.locate_protocol)(&mut guid, ptr::null_mut(), protocol.as_mut_ptr());

        result::from_value_and_status(unsafe { protocol.assume_init() }, s)
    }
}
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
