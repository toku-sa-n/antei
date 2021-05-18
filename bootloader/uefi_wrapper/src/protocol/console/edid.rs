use r_efi::efi;

#[repr(C)]
#[derive(Debug)]
pub struct Discovered {
    size: u32,
    info: *const u8,
}
unsafe impl crate::Protocol for Discovered {
    const GUID: efi::Guid = efi::Guid::from_fields(
        0x1c0c_34f6,
        0xd380,
        0x41fa,
        0xa0,
        0x49,
        &[0x8a, 0xd0, 0x6c, 0x1a, 0x66, 0xaa],
    );
}
