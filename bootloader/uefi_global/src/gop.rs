use uefi_wrapper::protocol::console::graphics_output::GraphicsOutput;
use uefi_wrapper::protocol::console::graphics_output::ModeInformation;
use uefi_wrapper::result::Result;

pub fn query_mode(mode_number: u32) -> Result<ModeInformation> {
    let mut st = crate::system_table();
    let bs = st.boot_services();
    let gop = bs.locate_protocol_without_registration::<GraphicsOutput>()?;

    gop.protocol.query_mode(mode_number)
}
