use core::fmt;
use r_efi::protocols::simple_text_output;

pub struct SimpleTextOutput<'a>(pub(crate) &'a simple_text_output::Protocol);
impl fmt::Debug for SimpleTextOutput<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimpleTextOutput").finish()
    }
}
