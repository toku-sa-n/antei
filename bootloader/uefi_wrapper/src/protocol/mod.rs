use core::fmt;
use r_efi::efi;

pub mod console;

/// # Safety
///
/// The type which implements thie trait must have the same structure as the UEFI specification
/// defines.
pub unsafe trait Protocol: fmt::Debug {
    const GUID: efi::Guid;
}
