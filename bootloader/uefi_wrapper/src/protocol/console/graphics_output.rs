use crate::protocol::Protocol;
use crate::result;
use crate::result::Result;
use core::fmt;
use core::mem;
use core::mem::MaybeUninit;
use r_efi::efi;
use r_efi::efi::protocols::graphics_output;

pub use r_efi::protocols::graphics_output::ModeInformation;

#[repr(transparent)]
pub struct GraphicsOutput(*mut graphics_output::Protocol);
impl GraphicsOutput {
    pub fn iter_mode(&mut self) -> impl Iterator<Item = ModeInformation> + '_ {
        ModeIter::from(self)
    }

    pub fn set_mode(&mut self, mode_number: u32) -> Result<()> {
        let s = (self.get_mut().set_mode)(self.0, mode_number);

        result::from_value_and_status((), s)
    }

    fn query_mode(&mut self, mode_number: u32) -> Result<ModeInformation> {
        let mut size = MaybeUninit::uninit();
        let mut info = MaybeUninit::uninit();

        let s =
            (self.get_mut().query_mode)(self.0, mode_number, size.as_mut_ptr(), info.as_mut_ptr());

        let size = unsafe { size.assume_init() };
        let info = unsafe { info.assume_init() };

        assert_eq!(
            size,
            mem::size_of::<ModeInformation>(),
            "The size of the information is not same as `ModeInformation`."
        );

        result::from_closure_and_status(|| unsafe { *info }, s)
    }

    fn max_mode(&self) -> u32 {
        let m = self.get_ref().mode;

        unsafe { (*m).max_mode }
    }

    fn get_ref(&self) -> &graphics_output::Protocol {
        // SAFETY: This type is created only through `Boot::locate_protocol_without_regstration`.
        unsafe { &*self.0 }
    }

    fn get_mut(&mut self) -> &mut graphics_output::Protocol {
        // SAFETY: This type is created only through `Boot::locate_protocol_without_regstration`.
        unsafe { &mut *self.0 }
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

struct ModeIter<'a> {
    gop: &'a mut GraphicsOutput,
    i: u32,
}
impl<'a> Iterator for ModeIter<'a> {
    type Item = ModeInformation;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.gop.max_mode() {
            None
        } else {
            let m = self.gop.query_mode(self.i);
            self.i += 1;

            m.ok().or_else(|| self.next())
        }
    }
}
impl<'a> From<&'a mut GraphicsOutput> for ModeIter<'a> {
    fn from(gop: &'a mut GraphicsOutput) -> Self {
        Self { gop, i: 0 }
    }
}
