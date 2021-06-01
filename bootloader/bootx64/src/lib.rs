#![no_std]

pub mod fs;
pub mod gop;
pub mod io;
pub mod mem;
pub mod panic;

use core::fmt;
use uefi_wrapper::protocols::console;
use uefi_wrapper::service;

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
}
