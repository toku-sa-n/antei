use crate::protocols::console;
use crate::service;
use core::fmt;
use r_efi::efi;

#[repr(transparent)]
#[allow(missing_copy_implementations)]
pub struct SystemTable(*mut efi::SystemTable);
impl SystemTable {
    #[must_use]
    pub fn boot_services(&mut self) -> service::Boot<'_> {
        self.into()
    }

    #[must_use]
    pub fn con_out(&mut self) -> console::SimpleTextOutput<'_> {
        self.into()
    }

    pub(crate) fn get_ptr(&self) -> *mut efi::SystemTable {
        self.0
    }
}
impl fmt::Debug for SystemTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SystemTable").finish()
    }
}
