use uefi_wrapper::protocol::console::edid;
use uefi_wrapper::result::Result;

/// # Errors
///
/// This method returns an `Err` value if EDID Discovered protocol is not found.
pub fn preferred_resolution() -> Result<Option<(u32, u32)>> {
    let mut st = crate::system_table();
    let bs = st.boot_services();
    let edid = bs.locate_protocol_without_registration::<edid::Discovered>()?;

    Ok(edid.protocol.preferred_resolution())
}
