use core::convert::TryFrom;
use core::convert::TryInto;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use r_efi::efi;

pub type Result<T> = core::result::Result<T, core::result::Result<NotSuccess, usize>>;

pub(crate) fn from_value_and_status<T>(value: T, status: efi::Status) -> Result<T> {
    if status == efi::Status::SUCCESS {
        Ok(value)
    } else {
        Err(status.try_into())
    }
}

const ERROR_BIT: usize = usize::MAX - (usize::MAX >> 1);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NotSuccess {
    Error(Error),
    Warning(Warning),
}
impl TryFrom<efi::Status> for NotSuccess {
    type Error = usize;

    fn try_from(s: efi::Status) -> core::result::Result<Self, usize> {
        let s = s.as_usize();

        if let Some(e) = FromPrimitive::from_usize(s) {
            Ok(Self::Error(e))
        } else if let Some(w) = FromPrimitive::from_usize(s) {
            Ok(Self::Warning(w))
        } else {
            Err(s)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive)]
#[repr(usize)]
#[allow(clippy::pub_enum_variant_names, clippy::enum_clike_unportable_variant)]
pub enum Error {
    LoadError = ERROR_BIT | 1,
    InvalidParameter = ERROR_BIT | 2,
    Unsupported = ERROR_BIT | 3,
    BadBufferSize = ERROR_BIT | 4,
    BufferTooSmall = ERROR_BIT | 5,
    NotReady = ERROR_BIT | 6,
    DeviceError = ERROR_BIT | 7,
    WriteProtected = ERROR_BIT | 8,
    OutOfResources = ERROR_BIT | 9,
    VolumeCorrupted = ERROR_BIT | 10,
    VolumeFull = ERROR_BIT | 11,
    NoMedia = ERROR_BIT | 12,
    MediaChanged = ERROR_BIT | 13,
    NotFound = ERROR_BIT | 14,
    AccessDenied = ERROR_BIT | 15,
    NoResponse = ERROR_BIT | 16,
    NoMapping = ERROR_BIT | 17,
    Timeout = ERROR_BIT | 18,
    NotStarted = ERROR_BIT | 19,
    AlreadyStarted = ERROR_BIT | 20,
    Aborted = ERROR_BIT | 21,
    IcmpError = ERROR_BIT | 22,
    TftpError = ERROR_BIT | 23,
    ProtocolError = ERROR_BIT | 24,
    IncompatibleVersion = ERROR_BIT | 25,
    SecurityViolation = ERROR_BIT | 26,
    CrcError = ERROR_BIT | 27,
    EndOfMedia = ERROR_BIT | 28,
    EndOfFile = ERROR_BIT | 31,
    InvalidLanguage = ERROR_BIT | 32,
    CompromisedData = ERROR_BIT | 33,
    IpAddressConflict = ERROR_BIT | 34,
    HttpError = ERROR_BIT | 35,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive)]
pub enum Warning {
    UnknownGlyph = 1,
    DeleteFailure = 2,
    WriteFailure = 3,
    BufferTooSmall = 4,
    StaleData = 5,
    FileSystem = 6,
    ResetRequired = 7,
}
