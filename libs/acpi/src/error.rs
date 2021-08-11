#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    RsdpWrongSignature,
    RsdpUnsupportedRevision,
    RsdpWrongChecksumOfFirst20Bytes,
    RsdpWrongChecksumOfFullBytes,
    XsdtWrongSignature,
    XsdtWrongRevision,
    XsdtInvalidEntryLength,
    XsdtWrongChecksum,
    FadtWrongSignature,
    FadtWrongMajorVersion,
    FadtWrongMinorVersion,
    FadtWrongChecksum,
    UnsupportedAddressSpaceId(u8),
}
