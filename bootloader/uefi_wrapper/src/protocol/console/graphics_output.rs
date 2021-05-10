use crate::protocol::Protocol;
use crate::result;
use crate::result::Result;
use core::fmt;
use core::mem;
use core::mem::MaybeUninit;
use r_efi::efi;
use r_efi::efi::protocols::graphics_output;

pub use r_efi::protocols::graphics_output::ModeInformation;

#[repr(transparent)]
#[allow(missing_copy_implementations)]
pub struct GraphicsOutput(*mut graphics_output::Protocol);
impl GraphicsOutput {
    /// # Errors
    ///
    /// This method may return an `Err` value in some situations, for exampel `mode_number` is not
    /// supported.
    pub fn set_mode(&mut self, mode_number: u32) -> Result<()> {
        let s = (self.get_mut().set_mode)(self.0, mode_number);

        result::from_value_and_status((), s)
    }

    /// # Errors
    ///
    /// This method may return an `Err` value in some situations, for example `mode_number` is not
    /// supported.
    ///
    /// # Panics
    ///
    /// This method panics if the size of returned information is not usual one.
    pub fn query_mode(&mut self, mode_number: u32) -> Result<ModeInformation> {
        let mut size = MaybeUninit::uninit();
        let mut info = MaybeUninit::uninit();

        let s =
            (self.get_mut().query_mode)(self.0, mode_number, size.as_mut_ptr(), info.as_mut_ptr());

        let size = unsafe { size.assume_init() };
        let info = unsafe { info.assume_init() };

        assert_eq!(
            size,
            mem::size_of::<ModeInformation>(),
            "The size of the information is not same as `ModeInformation`."
        );

        result::from_closure_and_status(|| unsafe { *info }, s)
    }

    fn get_mut(&mut self) -> &mut graphics_output::Protocol {
        // SAFETY: This type is created only through `Boot::locate_protocol_without_regstration`.
        unsafe { &mut *self.0 }
    }
}
impl fmt::Debug for GraphicsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GraphicsOutput").finish()
    }
}
unsafe impl Protocol for GraphicsOutput {
    const GUID: efi::Guid = graphics_output::PROTOCOL_GUID;
}
