#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate rlibc as _;

mod allocator;
pub mod elf;
mod exit_boot_services;
pub mod fs;
pub mod gop;
pub mod io;
pub mod kernel;
mod mapper;
pub mod paging;
pub mod panic;
pub mod system_table;

pub(crate) type NumOfPages<T = Size4KiB> = os_units::NumOfPages<T>;

use x86_64::structures::paging::Size4KiB;
pub(crate) use {allocator::Allocator, mapper::Mapper};
pub use {exit_boot_services::exit_boot_services_and_return_mmap, system_table::SystemTable};
