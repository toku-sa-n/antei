#![no_std]

mod error;
pub mod message;
pub mod syscalls;

pub use {
    error::Error,
    message::Message,
    syscalls::{receive, send, try_receive, try_send, ReceiveFrom},
};
