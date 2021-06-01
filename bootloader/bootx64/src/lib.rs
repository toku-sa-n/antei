#![no_std]

pub mod fs;
pub mod gop;
pub mod io;
pub mod panic;

use uefi_wrapper::protocols::console;
use uefi_wrapper::service;

#[repr(transparent)]
pub struct SystemTable(uefi_wrapper::SystemTable);
impl SystemTable {
    pub fn boot_services(&mut self) -> service::Boot<'_> {
        self.0.boot_services()
    }

    pub fn con_out(&mut self) -> console::SimpleTextOutput<'_> {
        self.0.con_out()
    }
}
