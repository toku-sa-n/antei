pub mod console;
pub mod media;

use r_efi::efi;

pub unsafe trait Protocol {
    const GUID: efi::Guid;
}
