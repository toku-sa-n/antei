pub mod console;
pub mod media;

use r_efi::efi;

/// # Safety
///
/// The GUID must be correct, otherwise it will use the wrong protocol, and may violate the memory
/// safety.
pub unsafe trait Protocol {
    const GUID: efi::Guid;
}
