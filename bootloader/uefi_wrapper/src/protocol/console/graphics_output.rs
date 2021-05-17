use core::fmt;
use r_efi::efi::protocols::graphics_output;

pub struct GraphicsOutput(graphics_output::Protocol);
impl fmt::Debug for GraphicsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GraphicsOutput").finish()
    }
}
