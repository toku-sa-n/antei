pub(super) fn main() -> ! {
    loop {
        log::info!("MAIN");
        x86_64::instructions::hlt();
    }
}
