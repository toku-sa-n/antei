use crate::is_aligned;
use crate::Error;
use crate::ERR_MSG;

/// # Safety
///
/// The caller must follow the rules of caling [`core::slice::from_raw_parts_mut`] except the
/// alignment requirements.
pub unsafe fn from_raw_parts_mut<'a, T>(data: *mut T, len: usize) -> &'a mut [T] {
    let r = try_from_raw_parts_mut(data, len);
    r.expect(ERR_MSG)
}

/// # Safety
///
/// The caller must follow the rules of calling [`core::slice::from_raw_parts_mut`] except the
/// alignment requirements.
pub unsafe fn try_from_raw_parts_mut<'a, T>(
    data: *mut T,
    len: usize,
) -> Result<&'a mut [T], Error> {
    if data.is_null() {
        Err(Error::Null)
    } else if is_aligned(data) {
        Ok(core::slice::from_raw_parts_mut(data, len))
    } else {
        Err(Error::NotAligned)
    }
}
