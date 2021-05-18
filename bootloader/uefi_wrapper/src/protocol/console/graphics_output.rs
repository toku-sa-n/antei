use crate::result;
use core::fmt;
use core::mem;
use r_efi::efi;
use r_efi::efi::protocols::graphics_output;

pub use r_efi::efi::protocols::graphics_output::ModeInformation;

#[repr(transparent)]
pub struct GraphicsOutput(graphics_output::Protocol);
impl GraphicsOutput {
    pub fn query_mode(&mut self, mode: u32) -> crate::Result<graphics_output::ModeInformation> {
        let mut size = mem::MaybeUninit::uninit();
        let mut info = mem::MaybeUninit::uninit();

        let r = (self.0.query_mode)(&mut self.0, mode, size.as_mut_ptr(), info.as_mut_ptr());

        result::from_closure_and_status(r, || {
            let info = unsafe { info.assume_init() };

            unsafe { *info }
        })
    }
}
unsafe impl crate::Protocol for GraphicsOutput {
    const GUID: efi::Guid = graphics_output::PROTOCOL_GUID;
}
impl fmt::Debug for GraphicsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GraphicsOutput").finish()
    }
}
