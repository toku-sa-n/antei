use super::file::File;
use crate::result;
use aligned_ptr::ptr;
use core::fmt;
use core::mem;
use r_efi::efi;
use r_efi::efi::protocols::simple_file_system;

#[repr(transparent)]
pub struct SimpleFileSystem(simple_file_system::Protocol);
impl SimpleFileSystem {
    /// # Errors
    ///
    /// Refer to the UEFI specification.
    pub fn open_volume(&mut self) -> crate::Result<File<'_>> {
        let mut root = mem::MaybeUninit::uninit();

        let r = (self.0.open_volume)(&mut self.0, root.as_mut_ptr());

        result::from_status_and_closure(r, move || {
            // SAFETY: `open_volume` initializes `root`.
            let root = unsafe { root.assume_init() };

            // SAFETY: There is only one instance of `SimpleFileSystem`, which is created by
            // `locate_protocol`. Therefore there is the only one mutable reference to the file
            // handler.
            File::new(unsafe { ptr::as_mut(root) }, self)
        })
    }
}
unsafe impl crate::Protocol for SimpleFileSystem {
    const GUID: efi::Guid = simple_file_system::PROTOCOL_GUID;
}
impl fmt::Debug for SimpleFileSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimpleFileSystem").finish()
    }
}
