use crate::status;
use r_efi::efi;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Error<T> {
    status: status::NotSuccess,
    value: T,
}
impl<T> Error<T> {
    pub fn status(&self) -> &status::NotSuccess {
        &self.status
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}
impl From<efi::Status> for Error<()> {
    fn from(s: efi::Status) -> Self {
        Self {
            status: s.into(),
            value: (),
        }
    }
}
