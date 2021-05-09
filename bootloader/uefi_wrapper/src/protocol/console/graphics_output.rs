use crate::protocol::Protocol;
use core::fmt;
use r_efi::efi;
use r_efi::efi::protocols::graphics_output;

#[repr(transparent)]
pub struct GraphicsOutput(*mut graphics_output::Protocol);
impl fmt::Debug for GraphicsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GraphicsOutput").finish()
    }
}
unsafe impl Protocol for GraphicsOutput {
    const GUID: efi::Guid = graphics_output::PROTOCOL_GUID;
}
