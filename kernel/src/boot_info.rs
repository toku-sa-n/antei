use {boot_info::BootInfo, conquer_once::spin::OnceCell};

static BOOT_INFO: OnceCell<BootInfo> = OnceCell::uninit();

pub(super) fn save(boot_info: BootInfo) {
    BOOT_INFO
        .try_init_once(|| boot_info)
        .expect("`BOOT_INFO` is already initialized.");
}

pub(super) fn get<'a>() -> &'a BootInfo {
    BOOT_INFO
        .try_get()
        .expect("`BOOT_INFO` is not initialized.")
}
