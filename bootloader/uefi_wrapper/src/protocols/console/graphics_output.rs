use crate::result;
use aligned_ptr::ptr;
use core::fmt;
use core::mem;
use r_efi::efi;
use r_efi::efi::protocols::graphics_output;

pub use r_efi::efi::protocols::graphics_output::ModeInformation;

#[repr(transparent)]
pub struct GraphicsOutput(graphics_output::Protocol);
impl GraphicsOutput {
    /// # Errors
    ///
    /// Refer to the UEFI specification.
    pub fn query_mode(&mut self, mode: u32) -> crate::Result<ModeInformation> {
        let mut size = mem::MaybeUninit::uninit();
        let mut info = mem::MaybeUninit::uninit();

        let r = (self.0.query_mode)(&mut self.0, mode, size.as_mut_ptr(), info.as_mut_ptr());

        result::from_status_and_closure(r, || {
            // SAFETY: `query_mode` initializes `info` on success.
            let info = unsafe { info.assume_init() };

            // SAFETY: The value that `info` points to is an instance of `ModeInformation`.
            unsafe { ptr::get(info) }
        })
    }

    /// # Errors
    ///
    /// Refer to the UEFI specification.
    pub fn set_mode(&mut self, mode: u32) -> crate::Result<()> {
        let r = (self.0.set_mode)(&mut self.0, mode);

        result::from_status_and_value(r, ())
    }

    #[must_use]
    pub fn max_mode(&self) -> u32 {
        // SAFETY: `locate_protocol` creates only one instance of `GraphicsOutput`. No other
        // pointers point to the `Mode` struct.
        unsafe { ptr::get(self.0.mode).max_mode }
    }
}
unsafe impl crate::Protocol for GraphicsOutput {
    const GUID: efi::Guid = graphics_output::PROTOCOL_GUID;
}
impl fmt::Debug for GraphicsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GraphicsOutput").finish()
    }
}
