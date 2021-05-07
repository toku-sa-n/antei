use r_efi::efi;

pub type Result<T> = core::result::Result<T, efi::Status>;

pub(crate) fn from_value_and_status<T>(value: T, status: efi::Status) -> Result<T> {
    if status == efi::Status::SUCCESS {
        Ok(value)
    } else {
        Err(status)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Warning {
    UnknownGlyph = 1,
    DeleteFailure = 2,
    WriteFailure = 3,
    BufferTooSmall = 4,
    StaleData = 5,
    FileSystem = 6,
    ResetRequired = 7,
}
