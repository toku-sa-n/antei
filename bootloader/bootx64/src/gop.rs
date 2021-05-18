use uefi_wrapper::protocol::console;
use uefi_wrapper::protocol::console::edid;
use uefi_wrapper::protocol::console::graphics_output;

/// # Panics
///
/// This method panics if there is no proper GOP mode.
#[must_use]
pub fn set_preferred_resolution() -> graphics_output::ModeInformation {
    let resolution = resolution_to_use();

    let mut st = crate::system_table();
    let bs = st.boot_services();
    let gop = bs.locate_protocol_without_registration::<console::GraphicsOutput>();
    let gop = gop.expect("The graphics output protocol is not implemented.");

    for i in 0..gop.protocol.max_mode() {
        let mode_info = gop.protocol.query_mode(i);
        if let Ok(mode_info) = mode_info {
            if (
                mode_info.horizontal_resolution,
                mode_info.vertical_resolution,
            ) == resolution
            {
                let r = gop.protocol.set_mode(i);
                r.expect("`GraphicsOutput::set_mode` failed.");

                return mode_info;
            }
        }
    }

    panic!("No proper GOP mode found.");
}

fn resolution_to_use() -> (u32, u32) {
    const DEFAULT_RESOLUTION: (u32, u32) = (800, 600);

    preferred_resolution().unwrap_or(DEFAULT_RESOLUTION)
}

fn preferred_resolution() -> Option<(u32, u32)> {
    let mut st = crate::system_table();
    let bs = st.boot_services();
    let d = bs
        .locate_protocol_without_registration::<edid::Discovered>()
        .ok()?;

    d.protocol.preferred_resolution()
}
