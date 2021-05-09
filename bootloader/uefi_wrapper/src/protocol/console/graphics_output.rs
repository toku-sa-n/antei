use crate::protocol::Protocol;
use crate::result;
use crate::result::Result;
use core::fmt;
use core::mem;
use core::mem::MaybeUninit;
use r_efi::efi;
use r_efi::efi::protocols::graphics_output;
use r_efi::efi::protocols::graphics_output::ModeInformation;

#[repr(transparent)]
pub struct GraphicsOutput(*mut graphics_output::Protocol);
impl GraphicsOutput {
    pub fn query_mode(&mut self, mode_number: u32) -> Result<ModeInformation> {
        let mut size = MaybeUninit::uninit();
        let mut info = MaybeUninit::uninit();

        let s = unsafe {
            ((*self.0).query_mode)(self.0, mode_number, size.as_mut_ptr(), info.as_mut_ptr())
        };

        let size = unsafe { size.assume_init() };
        let info = unsafe { info.assume_init() };

        assert_eq!(
            size,
            mem::size_of::<ModeInformation>(),
            "The size of the information is not same as `ModeInformation`."
        );

        result::from_closure_and_status(|| unsafe { *info }, s)
    }
}
impl fmt::Debug for GraphicsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GraphicsOutput").finish()
    }
}
unsafe impl Protocol for GraphicsOutput {
    const GUID: efi::Guid = graphics_output::PROTOCOL_GUID;
}
