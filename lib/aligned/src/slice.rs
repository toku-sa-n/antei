use crate::is_aligned;
use crate::Error;
use crate::ERR_MSG;

/// # Safety
///
/// The caller must follow the rules of caling [`core::slice::from_raw_parts_mut`] except the
/// alignment requirements.
pub unsafe fn from_raw_parts_mut<'a, T>(data: *mut T, len: usize) -> &'a mut [T] {
    // SAFETY: The caller must uphold the all safety rules.
    let r = unsafe { try_from_raw_parts_mut(data, len) };
    r.expect(ERR_MSG)
}

/// # Safety
///
/// The caller must follow the rules of calling [`core::slice::from_raw_parts_mut`] except the
/// alignment requirements.
///
/// # Errors
///
/// This method may return an error:
///
/// - [`Error::Null`] - `p` is null.
/// - [`Error::NotAligned`] - `p` is not aligned correctly.
pub unsafe fn try_from_raw_parts_mut<'a, T>(
    data: *mut T,
    len: usize,
) -> Result<&'a mut [T], Error> {
    if data.is_null() {
        Err(Error::Null)
    } else if is_aligned(data) {
        // SAFETY: The caller must uphold the all safety rules.
        Ok(unsafe { core::slice::from_raw_parts_mut(data, len) })
    } else {
        Err(Error::NotAligned)
    }
}
