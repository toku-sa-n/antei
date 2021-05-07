use crate::result;
use crate::result::Result;
use core::fmt;
use r_efi::protocols::simple_text_output;

pub struct SimpleTextOutput<'a>(pub(crate) &'a mut simple_text_output::Protocol);
impl SimpleTextOutput<'_> {
    /// # Errors
    ///
    /// This method may return an error if the output device is not functioning.
    pub fn reset_without_extension(&mut self) -> Result<()> {
        let s = (self.0.reset)(self.0, false.into());
        result::from_value_and_status((), s)
    }

    /// # Errors
    ///
    /// This method may return an error if the output device is not functioning.
    pub fn output_string(&mut self, buf: &mut [u16]) -> Result<()> {
        let s = (self.0.output_string)(self.0, buf.as_mut_ptr());
        result::from_value_and_status((), s)
    }
}
impl fmt::Debug for SimpleTextOutput<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimpleTextOutput").finish()
    }
}
