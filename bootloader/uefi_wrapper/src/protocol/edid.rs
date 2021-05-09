use crate::protocol::Protocol;
use r_efi::efi::Guid;

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Discovered([u8; 128]);
impl Discovered {
    #[must_use]
    pub fn preferred_resolution(&self) -> (u32, u32) {
        (self.preferred_resolution_x(), self.preferred_resolution_y())
    }

    fn preferred_resolution_x(&self) -> u32 {
        let upper = (u32::from(self.0[58]) & 0xf0) << 4;
        let lower: u32 = self.0[56].into();

        upper | lower
    }

    fn preferred_resolution_y(&self) -> u32 {
        let upper = (u32::from(self.0[61]) & 0xf0) << 4;
        let lower: u32 = self.0[59].into();

        upper | lower
    }
}
unsafe impl Protocol for Discovered {
    const GUID: Guid = Guid::from_fields(
        0x1c0c_34f6,
        0xd380,
        0x41fa,
        0xa0,
        0x49,
        &[0x8a, 0xd0, 0x6c, 0x1a, 0x66, 0xaa],
    );
}
