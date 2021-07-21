use crate::protocols::console;
use crate::service;
use crate::service::boot;
use aligned_ptr::ptr;
use core::fmt;
use r_efi::efi;

#[repr(transparent)]
#[allow(missing_copy_implementations)]
pub struct SystemTable(*mut efi::SystemTable);
impl SystemTable {
    #[must_use]
    pub fn boot_services(&self) -> service::Boot<'_> {
        // SAFETY: `st.boot_services` points to the instance of `efi::BootServices`.
        //
        // A value of `SystemTable` is created only through the argument of `efi_main`. Since this method
        // takes a mutable reference and this type does not implement `Copy`, only one mutable
        // reference to `efi::BootServices` is created.
        let bs = unsafe { ptr::as_ref(self.as_ref().boot_services) };

        service::Boot::new(bs)
    }

    #[must_use]
    pub fn con_out(&mut self) -> console::SimpleTextOutput<'_> {
        let st = self.as_mut();

        // SAFETY: `st.con_out` points to the instance of `efi::SimpleTextOutput`. A value of
        // `SystemTable` is created only through the argument of `efi_main`. Since this method
        // takes a mutable reference and this type does not implement `Copy`, only one mutable
        // reference to `efi::simple_text_output::Protocol` is created.
        let con_out = unsafe { ptr::as_mut(st.con_out) };

        console::SimpleTextOutput::new(con_out)
    }

    /// # Errors
    ///
    /// Refer to the UEFI specification.
    pub fn exit_boot_services(
        self,
        image_handle: crate::Handle,
        map_key: boot::MapKey,
    ) -> crate::Result<(), (Self, crate::Handle)> {
        // SAFETY: `Self` is created only through the argument of `efi_main`.
        let st = unsafe { ptr::as_mut(self.0) };
        // SAFETY: Same as the above comment.
        let bs = unsafe { ptr::as_mut(st.boot_services) };

        let s = (bs.exit_boot_services)(image_handle.get_ptr(), map_key.into());

        if s == efi::Status::SUCCESS {
            Ok(())
        } else {
            Err(crate::Error::from_status_and_value(
                s.into(),
                (self, image_handle),
            ))
        }
    }

    fn as_mut(&mut self) -> &mut efi::SystemTable {
        // SAFETY: `self.0` points to the instance of `efi::SystemTable`.
        //
        // A value of `SystemTable` is created only through the argument of `efi_main`. Since this method takes a mutable
        // reference of an instance and this type does not implement `Copy`, only one mutable reference to `efi::SystemTable` is created.
        unsafe { ptr::as_mut(self.0) }
    }

    fn as_ref(&self) -> &efi::SystemTable {
        // SAFETY: A value of `SystemTable` is created only through the argument of `efi_main`. The
        // UEFI firmware must pass the correct pointer.
        unsafe { ptr::as_ref(self.0) }
    }
}
impl fmt::Debug for SystemTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SystemTable").finish()
    }
}
