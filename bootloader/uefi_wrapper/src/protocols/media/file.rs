use r_efi::efi::protocols::file;

pub struct File<'a>(&'a mut file::Protocol);
impl<'a> From<&'a mut file::Protocol> for File<'a> {
    fn from(f: &'a mut file::Protocol) -> Self {
        Self(f)
    }
}
