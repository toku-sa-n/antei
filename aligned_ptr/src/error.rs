//! Errors.

/// Errors returned by functions of this crate.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    /// The passed pointer is null.
    Null,
    /// The passed pointer is not aligned correctly.
    NotAligned,
}
