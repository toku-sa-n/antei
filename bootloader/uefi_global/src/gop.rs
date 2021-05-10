use uefi_wrapper::protocol::console::graphics_output::GraphicsOutput;
use uefi_wrapper::result::Result;

pub use uefi_wrapper::protocol::console::graphics_output::ModeInformation;

/// # Errors
///
/// This function may return an `Err` value in some situations, for example GOP is not supported.
pub fn query_mode(mode_number: ModeNumber) -> Result<ModeInformation> {
    let mut st = crate::system_table();
    let bs = st.boot_services();
    let gop = bs.locate_protocol_without_registration::<GraphicsOutput>()?;

    gop.protocol.query_mode(mode_number.into())
}

/// # Errors
///
/// This function may return an `Err` value in some situations, for example GOP is not supported.
pub fn set_mode(mode_number: ModeNumber) -> Result<()> {
    let mut st = crate::system_table();
    let bs = st.boot_services();
    let gop = bs.locate_protocol_without_registration::<GraphicsOutput>()?;

    gop.protocol.set_mode(mode_number.into())
}

/// # Errors
///
/// This function may return an `Err` value in some situations, for example GOP is not supported.
pub fn max_mode() -> Result<u32> {
    let mut st = crate::system_table();
    let bs = st.boot_services();
    let gop = bs.locate_protocol_without_registration::<GraphicsOutput>()?;

    Ok(gop.protocol.max_mode())
}

#[derive(Copy, Clone, Debug)]
pub struct ModeNumber(u32);
impl ModeNumber {
    pub fn new(n: u32) -> Option<Self> {
        if n < max_mode().ok()? {
            Some(Self(n))
        } else {
            None
        }
    }
}
impl From<ModeNumber> for u32 {
    fn from(m: ModeNumber) -> Self {
        m.0
    }
}
