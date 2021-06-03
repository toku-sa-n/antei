#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

mod exit_boot_services;
pub mod fs;
pub mod gop;
pub mod io;
pub mod panic;

use core::fmt;
use uefi_wrapper::service;
use uefi_wrapper::{protocols::console, service::boot};

pub use exit_boot_services::exit_boot_services_and_return_mmap;

#[repr(transparent)]
#[derive(Debug)]
pub struct SystemTable(uefi_wrapper::SystemTable);
impl SystemTable {
    pub fn boot_services(&mut self) -> service::Boot<'_> {
        self.0.boot_services()
    }

    pub fn con_out(&mut self) -> console::SimpleTextOutput<'_> {
        self.0.con_out()
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

    fn exit_boot_services(
        self,
        image_handler: uefi_wrapper::Handle,
        map_key: boot::MapKey,
    ) -> uefi_wrapper::Result<(), (Self, uefi_wrapper::Handle)> {
        let r = self.0.exit_boot_services(image_handler, map_key);

        r.map_err(|e| e.map_value(|(st, h)| (Self(st), h)))
    }
}
