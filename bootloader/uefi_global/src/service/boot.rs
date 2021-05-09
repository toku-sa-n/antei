use core::ffi::c_void;
use r_efi::efi;
use uefi_wrapper::result::Result;

pub fn locate_protocol_without_registration(guid: efi::Guid) -> Result<*mut c_void> {
    crate::boot_services().locate_protocol_without_registration(guid)
}
