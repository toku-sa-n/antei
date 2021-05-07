use core::fmt;
use r_efi::protocols::simple_text_output;

pub struct SimpleTextOutput<'a>(pub(crate) &'a mut simple_text_output::Protocol);
impl<'a> SimpleTextOutput<'a> {
    pub fn output_string(&mut self, buf: &mut [u16]) {
        (self.0.output_string)(self.0, buf.as_mut_ptr());
    }
}
impl fmt::Debug for SimpleTextOutput<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimpleTextOutput").finish()
    }
}
