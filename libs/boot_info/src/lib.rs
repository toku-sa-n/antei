#![no_std]

const MAGIC_HEADER: u64 = 0x0114_0514_1919_0810;
const MAGIC_FOOTER: u64 = 0x0334_0072_dead_cafe;

#[repr(C)]
pub struct BootInfo {
    magic_header: u64,
    magic_footer: u64,
}
impl BootInfo {
    pub fn new() -> Self {
        Self {
            magic_header: MAGIC_HEADER,
            magic_footer: MAGIC_FOOTER,
        }
    }

    pub fn check_header_and_footer(&self) {
        self.check_header();
        self.check_footer();
    }

    fn check_header(&self) {
        assert_eq!(
            self.magic_header, MAGIC_HEADER,
            "Invalid boot information header."
        );
    }

    fn check_footer(&self) {
        assert_eq!(
            self.magic_footer, MAGIC_FOOTER,
            "Invalid boot information footer."
        );
    }
}
