#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod error;
pub mod handle;
pub mod protocols;
pub mod result;
pub mod service;
pub mod status;
pub mod system_table;

pub use error::Error;
pub use handle::Handle;
pub use protocols::Protocol;
pub use result::Result;
pub use system_table::SystemTable;
