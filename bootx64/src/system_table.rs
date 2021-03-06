use {
    crate::uefi_panic,
    core::fmt,
    uefi::{
        protocols::console,
        service::{self, boot},
        system_table::ConfigurationTable,
    },
};

#[repr(transparent)]
#[derive(Debug)]
pub struct SystemTable(uefi::SystemTable);
impl SystemTable {
    pub fn boot_services(&mut self) -> service::Boot<'_> {
        self.0.boot_services()
    }

    pub fn con_out(&mut self) -> console::SimpleTextOutput<'_> {
        self.0.con_out()
    }

    #[must_use]
    pub fn configuration_table(&self) -> &[ConfigurationTable] {
        self.0.configuration_table()
    }

    /// # Panics
    ///
    /// This method panics if `result` is [`Err`].
    pub fn expect_ok<T, E: fmt::Debug>(&mut self, result: Result<T, E>, msg: &str) -> T {
        match result {
            Ok(val) => val,
            Err(e) => {
                uefi_panic!(self, "{}: {:?}", msg, e);
            }
        }
    }

    pub(crate) fn exit_boot_services(
        self,
        image_handler: uefi::Handle,
        map_key: boot::MapKey,
    ) -> uefi::Result<(), (Self, uefi::Handle)> {
        let r = self.0.exit_boot_services(image_handler, map_key);

        r.map_err(|e| e.map_value(|(st, h)| (Self(st), h)))
    }
}
