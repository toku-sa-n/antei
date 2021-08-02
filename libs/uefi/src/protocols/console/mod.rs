pub mod edid;
pub mod graphics_output;
pub mod simple_text_output;

use r_efi::efi;

pub use graphics_output::GraphicsOutput;
pub use simple_text_output::SimpleTextOutput;

/// # Safety
///
/// The type that implements this trait must have the same structure as defined in the UEFI
/// specification.
pub unsafe trait Protocol {
    const GUID: efi::Guid;
}
