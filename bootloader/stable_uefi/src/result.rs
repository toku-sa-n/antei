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

const ERROR_MASK: usize = usize::MAX - (usize::MAX >> 1);

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
pub enum Error {
    LoadError = ERROR_MASK | 1,
    InvalidParameter = ERROR_MASK | 2,
    Unsupported = ERROR_MASK | 3,
    BadBufferSize = ERROR_MASK | 4,
    BufferTooSmall = ERROR_MASK | 5,
    NotReady = ERROR_MASK | 6,
    DeviceError = ERROR_MASK | 7,
    WriteProtected = ERROR_MASK | 8,
    OutOfResources = ERROR_MASK | 9,
    VolumeCorrupted = ERROR_MASK | 10,
    VolumeFull = ERROR_MASK | 11,
    NoMedia = ERROR_MASK | 12,
    MediaChanged = ERROR_MASK | 13,
    NotFound = ERROR_MASK | 14,
    AccessDenied = ERROR_MASK | 15,
    NoResponse = ERROR_MASK | 16,
    NoMapping = ERROR_MASK | 17,
    Timeout = ERROR_MASK | 18,
    NotStarted = ERROR_MASK | 19,
    AlreadyStarted = ERROR_MASK | 20,
    Aborted = ERROR_MASK | 21,
    IcmpError = ERROR_MASK | 22,
    TftpError = ERROR_MASK | 23,
    ProtocolError = ERROR_MASK | 24,
    IncompatibleVersion = ERROR_MASK | 25,
    SecurityViolation = ERROR_MASK | 26,
    CrcError = ERROR_MASK | 27,
    EndOfMedia = ERROR_MASK | 28,
    EndOfFile = ERROR_MASK | 31,
    InvalidLanguage = ERROR_MASK | 32,
    CompromisedData = ERROR_MASK | 33,
    IpAddressConflict = ERROR_MASK | 34,
    HttpError = ERROR_MASK | 35,
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
