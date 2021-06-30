#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

mod allocator;
pub mod elf;
mod exit_boot_services;
pub mod fs;
pub mod gop;
pub mod io;
mod mapper;
pub mod paging;
pub mod panic;
pub mod system_table;

pub(crate) use allocator::Allocator;
pub use exit_boot_services::exit_boot_services_and_return_mmap;
pub(crate) use mapper::Mapper;
pub use system_table::SystemTable;

use x86_64::VirtAddr;

pub fn jump_to_kernel(entry: VirtAddr) -> ! {
    // SAFETY: Safe as described in
    // https://rust-lang.github.io/unsafe-code-guidelines/layout/function-pointers.html#representation.
    let entry: fn() -> ! = unsafe { core::mem::transmute(entry.as_ptr::<()>()) };

    (entry)()
}
