use r_efi::efi;
pub mod console;

pub unsafe trait Protocol {
    const GUID: efi::Guid;
}
