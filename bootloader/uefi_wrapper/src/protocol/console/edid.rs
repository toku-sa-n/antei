use crate::protocol::Protocol;
use core::convert::TryInto;
use core::slice;
use r_efi::efi::Guid;

#[repr(C)]
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct Discovered {
    size: u32,
    ptr: *const u8,
}
impl Discovered {
    #[must_use]
    pub fn preferred_resolution(&self) -> Option<(u32, u32)> {
        Some((
            self.preferred_resolution_x()?,
            self.preferred_resolution_y()?,
        ))
    }

    fn preferred_resolution_x(&self) -> Option<u32> {
        let info = self.info()?;

        let upper = (u32::from(info[58]) & 0xf0) << 4;
        let lower: u32 = info[56].into();

        Some(upper | lower)
    }

    fn preferred_resolution_y(&self) -> Option<u32> {
        let info = self.info()?;

        let upper = (u32::from(info[61]) & 0xf0) << 4;
        let lower: u32 = info[59].into();

        Some(upper | lower)
    }

    fn info(&self) -> Option<&[u8]> {
        if self.ptr.is_null() {
            None
        } else {
            Some(unsafe { slice::from_raw_parts(self.ptr, self.size.try_into().unwrap()) })
        }
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
