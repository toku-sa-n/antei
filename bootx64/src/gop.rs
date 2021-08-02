use crate::uefi_panic;
use uefi::protocols::console::edid;
use uefi::protocols::console::graphics_output;
use uefi::{protocols::console, service::boot::WithProtocol};

/// # Panics
///
/// This method panics if there is no proper GOP mode.
#[must_use]
pub fn set_preferred_resolution(st: &mut crate::SystemTable) -> graphics_output::ModeInformation {
    let s = try_set_preferred_resolution(st);
    st.expect_ok(s, "Failed to set the preferred screen resolution.")
}

fn try_set_preferred_resolution(
    st: &mut crate::SystemTable,
) -> uefi::Result<graphics_output::ModeInformation> {
    let resolution = resolution_to_use(st);

    let gop = try_get_gop(st)?.protocol;

    for i in 0..gop.max_mode() {
        let mode_info = gop.query_mode(i);
        if let Ok(mode_info) = mode_info {
            if (
                mode_info.horizontal_resolution,
                mode_info.vertical_resolution,
            ) == resolution
            {
                gop.set_mode(i)?;

                return Ok(mode_info);
            }
        }
    }

    uefi_panic!(st, "No proper GOP mode found.");
}

fn try_get_gop(
    st: &mut crate::SystemTable,
) -> uefi::Result<WithProtocol<'_, console::GraphicsOutput>> {
    let bs = st.boot_services();

    bs.locate_protocol_without_registration::<console::GraphicsOutput>()
}

fn resolution_to_use(st: &mut crate::SystemTable) -> (u32, u32) {
    const DEFAULT_RESOLUTION: (u32, u32) = (800, 600);

    preferred_resolution(st).unwrap_or(DEFAULT_RESOLUTION)
}

fn preferred_resolution(st: &mut crate::SystemTable) -> Option<(u32, u32)> {
    let bs = st.boot_services();
    let d = bs
        .locate_protocol_without_registration::<edid::Discovered>()
        .ok()?;

    d.protocol.preferred_resolution()
}
