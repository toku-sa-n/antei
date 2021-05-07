use r_efi::efi;

pub type Result<T> = core::result::Result<T, efi::Status>;

pub(crate) fn from_value_and_status<T>(value: T, status: efi::Status) -> Result<T> {
    if status == efi::Status::SUCCESS {
        Ok(value)
    } else {
        Err(status)
    }
}
