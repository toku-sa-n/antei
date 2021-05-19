use r_efi::efi::protocols::simple_file_system;

#[repr(transparent)]
pub struct SimpleFileSystem(simple_file_system::Protocol);
