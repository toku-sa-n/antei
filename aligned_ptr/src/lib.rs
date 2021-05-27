//! A library to ensure that a pointer is aligned and not null when it dereferences.

#![no_std]

pub mod error;

use core::mem;
pub use error::Error;

/// Gets a value the pointer `p` points by dereferencing it.
///
/// # Safety
///
/// The pointer `p` must follow these rules:
///
/// - It must be dereferencable as defined in [Rust's documentation](https://doc.rust-lang.org/std/ptr/index.html#safety).
/// - It must point to the initialized instance of T.
///
/// Also, the caller must follow Rust's aliasing rule. The caller must not create both immutable
/// and mutable references, or multiple mutable references to the same object.
///
/// # Panics
///
/// This method panics if `p` is either null or not aligned correctly.
pub unsafe fn get<T: Copy>(p: *const T) -> T {
    try_get(p).expect("Pointer is either null or not aligned.")
}

/// Gets a value the pointer `p` points by dereferencing it.
///
/// # Safety
///
/// The pointer `p` must follow these rules:
///
/// - It must be dereferencable as defined in [Rust's documentation](https://doc.rust-lang.org/std/ptr/index.html#safety).
/// - It must point to the initialized instance of T.
///
/// Also, the caller must follow Rust's aliasing rule. The caller must not create both immutable
/// and mutable references, or multiple mutable references to the same object.
///
/// # Errors
///
/// This method may return an error:
///
/// - [`Error::Null`] - `p` is null.
/// - [`Error::NotAligned`] - `p` is not aligned correctly.
pub unsafe fn try_get<T: Copy>(p: *const T) -> Result<T, Error> {
    if p.is_null() {
        Err(Error::Null)
    } else if is_aligned(p) {
        Ok(*p)
    } else {
        Err(Error::NotAligned)
    }
}

/// Converts a pointer to a mutable reference.
///
/// # Safety
///
/// The pointer `p` must follow these rules:
///
/// - It must be dereferencable as defined in [Rust's documentation](https://doc.rust-lang.org/std/ptr/index.html#safety).
/// - It must point to the initialized instance of T.
///
/// Also, the caller must follow Rust's aliasing rule. The caller must not create both immutable
/// and mutable references, or multiple mutable references to the same object.
///
/// # Panics
///
/// This method panics if `p` is either null or not aligned correctly.
pub unsafe fn as_mut<'a, T>(p: *mut T) -> &'a mut T {
    try_as_mut(p).expect("Pointer is either null or not aligned.")
}

/// Converts a pointer to a mutable reference.
///
/// # Safety
///
/// The pointer `p` must follow these rules:
///
/// - It must be dereferencable as defined in [Rust's documentation](https://doc.rust-lang.org/std/ptr/index.html#safety).
/// - It must point to the initialized instance of T.
///
/// Also, the caller must follow Rust's aliasing rule. The caller must not create both immutable
/// and mutable references, or multiple mutable references to the same object.
///
/// # Errors
///
/// This method may return an error:
///
/// - [`Error::Null`] - `p` is null.
/// - [`Error::NotAligned`] - `p` is not aligned correctly.
pub unsafe fn try_as_mut<'a, T>(p: *mut T) -> Result<&'a mut T, Error> {
    if p.is_null() {
        Err(Error::Null)
    } else if is_aligned(p) {
        Ok(&mut *p)
    } else {
        Err(Error::NotAligned)
    }
}

/// Converts a pointer to an immutable reference.
///
/// # Safety
///
/// The pointer `p` must follow these rules:
///
/// - It must be dereferencable as defined in [Rust's documentation](https://doc.rust-lang.org/std/ptr/index.html#safety).
/// - It must point to the initialized instance of T.
///
/// Also, the caller must follow Rust's aliasing rule. The caller must not create both immutable
/// and mutable references to the same object simultaneously.
///
/// # Panics
///
/// This method panics if `p` is either null or not aligned correctly.
pub unsafe fn as_ref<'a, T>(p: *const T) -> &'a T {
    try_as_ref(p).expect("Pointer is either null or not aligned.")
}

/// Converts a pointer to an immutable reference.
///
/// # Safety
///
/// The pointer `p` must follow these rules:
///
/// - It must be dereferencable as defined in [Rust's documentation](https://doc.rust-lang.org/std/ptr/index.html#safety).
/// - It must point to the initialized instance of T.
///
/// Also, the caller must follow Rust's aliasing rule. The caller must not create both immutable
/// and mutable references to the same object simultaneously.
///
/// # Errors
///
/// This method may return an error:
///
/// - [`Error::Null`] - `p` is null.
/// - [`Error::NotAligned`] - `p` is not aligned correctly.
pub unsafe fn try_as_ref<'a, T>(p: *const T) -> Result<&'a T, Error> {
    if p.is_null() {
        Err(Error::Null)
    } else if is_aligned(p) {
        Ok(&*p)
    } else {
        Err(Error::NotAligned)
    }
}

fn is_aligned<T>(p: *const T) -> bool {
    p as usize % mem::align_of::<T>() == 0
}
