use crate::protocol::console;
use crate::service;
use core::fmt;
use r_efi::efi;

#[repr(transparent)]
pub struct SystemTable(efi::SystemTable);
impl SystemTable {
    #[must_use]
    pub fn boot_services(&mut self) -> service::Boot<'_> {
        // SAFETY: `SystemTable` is created only from the argument of `efi_main`. We must trust the
        // argument is a valid pointer.
        service::Boot(unsafe { &mut *self.0.boot_services })
    }

    #[must_use]
    pub fn con_out(&mut self) -> console::SimpleTextOutput<'_> {
        // SAFETY: `SystemTable` is created only from the argument of `efi_main`. We must trust the
        // argument is a valid pointer.
        console::SimpleTextOutput(unsafe { &mut *self.0.con_out })
    }
}
impl fmt::Debug for SystemTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SystemTable").finish()
    }
}
