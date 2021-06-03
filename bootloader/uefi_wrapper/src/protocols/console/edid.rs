use core::{convert::TryInto, slice};
use r_efi::efi;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Discovered {
    size: u32,
    ptr: *const u8,
}
impl Discovered {
    #[must_use]
    pub fn preferred_resolution(&self) -> Option<(u32, u32)> {
        Some((self.preferred_width()?, self.preferred_height()?))
    }

    fn preferred_width(&self) -> Option<u32> {
        let info = self.get_info()?;

        let upper = (u32::from(info[58]) & 0xf0) << 4;
        let lower: u32 = info[56].into();

        Some(upper | lower)
    }

    fn preferred_height(&self) -> Option<u32> {
        let info = self.get_info()?;

        let upper = (u32::from(info[61]) & 0xf0) << 4;
        let lower: u32 = info[59].into();

        Some(upper | lower)
    }

    fn get_info(&self) -> Option<&[u8]> {
        if self.info_exists() {
            // SAFETY: The EDID Discovered information exists.
            Some(unsafe { self.get_info_unchecked() })
        } else {
            None
        }
    }

    unsafe fn get_info_unchecked(&self) -> &[u8] {
        let sz: usize = self.size.try_into().unwrap();

        // SAFETY: `self.ptr` is valid for `sz` bytes as it is not null. These memory are not
        // modified.
        unsafe { slice::from_raw_parts(self.ptr, sz) }
    }

    fn info_exists(&self) -> bool {
        !self.ptr.is_null()
    }
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
