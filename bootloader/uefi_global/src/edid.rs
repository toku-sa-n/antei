use uefi_wrapper::protocol::edid;
use uefi_wrapper::result::Result;

pub fn preferred_resolution() -> Result<(u32, u32)> {
    let mut st = crate::system_table();
    let bs = st.boot_services();
    let edid = bs.locate_protocol_without_registration::<edid::Discovered>()?;

    Ok(edid.protocol.preferred_resolution())
}
