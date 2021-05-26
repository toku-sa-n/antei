//! Errors.

/// Errors returned by functions of this crate.
pub enum Error {
    /// The passed pointer is null.
    Null,
    /// The passed pointer is not aligned correctly.
    NotAligned,
}
