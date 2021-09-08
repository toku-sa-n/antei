#![no_std]

pub mod message;
pub mod syscalls;

pub use {
    message::Message,
    syscalls::{receive, send, ReceiveFrom},
};
