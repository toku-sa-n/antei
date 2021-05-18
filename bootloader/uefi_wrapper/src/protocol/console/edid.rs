#[repr(C)]
#[derive(Debug)]
pub struct Discovered {
    size: u32,
    info: *const u8,
}
