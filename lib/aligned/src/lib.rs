//! A library to ensure that a pointer is aligned and not null when it dereferences.

#![no_std]

pub mod error;
pub mod ptr;
pub mod slice;

pub use error::Error;

fn is_aligned<T>(p: *const T) -> bool {
    p as usize % core::mem::align_of::<T>() == 0
}
