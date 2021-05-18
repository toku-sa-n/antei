use uefi_wrapper::protocol::console::edid;

pub fn preferred_resolution() -> Option<(u32, u32)> {
    let mut st = crate::system_table();
    let bs = st.boot_services();
    let d = bs
        .locate_protocol_without_registration::<edid::Discovered>()
        .ok()?;

    d.protocol.preferred_resolution()
}
