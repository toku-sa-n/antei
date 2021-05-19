use r_efi::efi;
use r_efi::efi::protocols::simple_file_system;

#[repr(transparent)]
pub struct SimpleFileSystem(simple_file_system::Protocol);
unsafe impl crate::Protocol for SimpleFileSystem {
    const GUID: efi::Guid = simple_file_system::PROTOCOL_GUID;
}
