use super::simple_file_system::SimpleFileSystem;
use r_efi::efi::protocols::file;

pub struct File<'a> {
    protocol: &'a mut file::Protocol,
    fs: &'a mut SimpleFileSystem,
}
impl<'a> File<'a> {
    pub(crate) fn new(protocol: &'a mut file::Protocol, fs: &'a mut SimpleFileSystem) -> Self {
        Self { protocol, fs }
    }
}
