use super::simple_file_system::SimpleFileSystem;
use crate::result;
use aligned_ptr::ptr;
use core::fmt;
use core::mem;
use r_efi::efi::protocols::file;
use r_efi::efi::Status;

const FAT_MAX_NAME: usize = 255;

pub struct File<'a> {
    handler: &'a mut file::Protocol,
    _fs: &'a mut SimpleFileSystem,
}
impl<'a> File<'a> {
    /// # Errors
    ///
    /// Refer to the UEFI specification.
    pub fn open_read_only(&mut self, name: &str) -> crate::Result<()> {
        if name_too_long(name) {
            Err(Status::INVALID_PARAMETER.into())
        } else {
            self.open_read_only_unchecked(name)
        }
    }

    /// # Errors
    ///
    /// Refer to the UEFI specification.
    pub fn set_position(&mut self, position: u64) -> crate::Result<()> {
        let r = (self.handler.set_position)(self.handler, position);

        result::from_status_and_value(r, ())
    }

    /// # Errors
    ///
    /// Refer to the UEFI specification.
    pub fn get_position(&mut self) -> crate::Result<u64> {
        let mut position = mem::MaybeUninit::uninit();
        let r = (self.handler.get_position)(self.handler, position.as_mut_ptr());

        result::from_status_and_closure(r, || {
            // SAFETY: `get_position` initializes `position`.
            unsafe { position.assume_init() }
        })
    }

    /// # Errors
    ///
    /// Refer to the UEFI specification.
    pub fn read(&mut self, buf: &mut [u8]) -> crate::Result<(), Option<usize>> {
        let mut buf_len = buf.len();
        let r = (self.handler.read)(self.handler, &mut buf_len, buf.as_mut_ptr().cast());

        match r {
            Status::SUCCESS => Ok(()),
            Status::BUFFER_TOO_SMALL => {
                Err(crate::Error::from_status_and_value(r.into(), Some(buf_len)))
            }
            _ => Err(crate::Error::from_status_and_value(r.into(), None)),
        }
    }

    pub(crate) fn new(handler: &'a mut file::Protocol, fs: &'a mut SimpleFileSystem) -> Self {
        Self { handler, _fs: fs }
    }

    fn open_read_only_unchecked(&mut self, name: &str) -> crate::Result<()> {
        const ATTRIBUTES_ARE_IGNORED: u64 = 0;

        let mut name = name_to_u16_array(name);
        let mut new_handler = mem::MaybeUninit::uninit();

        let r = (self.handler.open)(
            self.handler,
            new_handler.as_mut_ptr(),
            name.as_mut_ptr(),
            file::READ_ONLY,
            ATTRIBUTES_ARE_IGNORED,
        );

        result::from_status_and_closure(r, move || {
            // SAFETY: `open` initializes `new_handler`.
            let new_handler = unsafe { new_handler.assume_init() };

            // SAFETY: Only one instance of `File` exists per `SimpleFileSystem`. Therefore there
            // is no mutable references which point to `*new_handler`.
            let new_handler = unsafe { ptr::as_mut(new_handler) };

            (self.handler.close)(self.handler);

            self.handler = new_handler;
        })
    }
}
impl Drop for File<'_> {
    fn drop(&mut self) {
        (self.handler.close)(self.handler);
    }
}
impl fmt::Debug for File<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("File")
    }
}

fn name_to_u16_array(name: &str) -> [u16; FAT_MAX_NAME + 1] {
    let mut buf = [0; FAT_MAX_NAME + 1];
    let r = ucs2::encode(name, &mut buf);

    r.expect("Failed to convert the file name to an u16 array.");

    buf
}

fn name_too_long(name: &str) -> bool {
    name.len() > FAT_MAX_NAME
}
