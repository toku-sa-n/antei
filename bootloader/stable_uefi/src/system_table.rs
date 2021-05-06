use crate::service;
use r_efi::efi;

#[repr(transparent)]
pub struct SystemTable(efi::SystemTable);
impl SystemTable {
    pub fn boot_services(&self) -> service::Boot<'_> {
        // SAFETY: `SystemTable` is created only from the argument of `efi_main`. We must trust the
        // argument is a valid pointer.
        service::Boot(unsafe { &*self.0.boot_services })
    }
}
