use r_efi::efi;

pub type Result<T, E = ()> = core::result::Result<T, crate::Error<E>>;

pub(crate) fn from_status_and_value<T>(status: efi::Status, value: T) -> Result<T, ()> {
    from_status_and_closure(status, || value)
}

pub(crate) fn from_status_and_closure<T>(
    status: efi::Status,
    f: impl FnOnce() -> T,
) -> Result<T, ()> {
    if status == efi::Status::SUCCESS {
        Ok(f())
    } else {
        Err(status.into())
    }
}
