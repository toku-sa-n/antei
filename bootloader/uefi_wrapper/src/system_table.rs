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
        let st = self.as_mut();

        // SAFETY: `st.boot_services` points to the instance of `efi::BootServices`.
        //
        // A value of `SystemTable` is created only through the argument of `efi_main`. Since this method
        // takes a mutable reference and this type does not implement `Copy`, only one mutable
        // reference to `efi::BootServices` is created.
        let bs = unsafe { aligned::as_mut(st.boot_services) };

        service::Boot::new(bs, self)
    }

    #[must_use]
    pub fn con_out(&mut self) -> console::SimpleTextOutput<'_> {
        let st = self.as_mut();

        // SAFETY: `st.con_out` points to the instance of `efi::SimpleTextOutput`. A value of
        // `SystemTable` is created only through the argument of `efi_main`. Since this method
        // takes a mutable reference and this type does not implement `Copy`, only one mutable
        // reference to `efi::simple_text_output::Protocol` is created.
        let con_out = unsafe { aligned::as_mut(st.con_out) };

        console::SimpleTextOutput::new(con_out, self)
    }

    fn as_mut(&mut self) -> &mut efi::SystemTable {
        // SAFETY: `self.0` points to the instance of `efi::SystemTable`.
        //
        // A value of `SystemTable` is created only through the argument of `efi_main`. Since this method takes a mutable
        // reference of an instance and this type does not implement `Copy`, only one mutable reference to `efi::SystemTable` is created.
        unsafe { aligned::as_mut(self.0) }
    }
}
impl fmt::Debug for SystemTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SystemTable").finish()
    }
}
