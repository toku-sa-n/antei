use crate::is_aligned;
use crate::Error;
use crate::ERR_MSG;

/// Reads a value the pointer `p` points with [`core::ptr::read`].
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
pub unsafe fn read<T>(p: *const T) -> T {
    // SAFETY: The caller must uphold the all safety rules.
    unsafe { try_read(p).expect(ERR_MSG) }
}

/// Reads a value the pointer `p` points with [`core::ptr::read`].
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
pub unsafe fn try_read<T>(p: *const T) -> Result<T, Error> {
    if p.is_null() {
        Err(Error::Null)
    } else if is_aligned(p) {
        // SAFETY: The caller must uphold the all safety rules.
        Ok(unsafe { p.read() })
    } else {
        Err(Error::NotAligned)
    }
}

/// Writes a value the pointer `p` points with [`core::ptr::write`].
///
/// # Safety
///
/// The pointer `p` must be dereferencable as defined in [Rust's documentation](https://doc.rust-lang.org/std/ptr/index.html#safety).
///
/// # Panics
///
/// This method panics if `p` is either null or not aligned correctly.
pub unsafe fn write<T>(p: *mut T, v: T) {
    // SAFETY: The caller must uphold the all safety rules.
    unsafe { try_write(p, v).expect(ERR_MSG) }
}

/// Writes a value the pointer `p` points with [`core::ptr::write`].
///
/// # Safety
///
/// The pointer `p`  must be dereferencable as defined in [Rust's documentation](https://doc.rust-lang.org/std/ptr/index.html#safety).
///
/// # Errors
///
/// This method may return an error:
///
/// - [`Error::Null`] - `p` is null.
/// - [`Error::NotAligned`] - `p` is not aligned correctly.
pub unsafe fn try_write<T>(p: *mut T, v: T) -> Result<(), Error> {
    if p.is_null() {
        Err(Error::Null)
    } else if is_aligned(p) {
        // SAFETY: The caller must uphold the all safety rules.
        unsafe { p.write(v) };
        Ok(())
    } else {
        Err(Error::NotAligned)
    }
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
/// # Panics
///
/// This method panics if `p` is either null or not aligned correctly.
pub unsafe fn get<T: Copy>(p: *const T) -> T {
    // SAFETY: The caller must uphold the all safety rules.
    unsafe { try_get(p).expect(ERR_MSG) }
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
        // SAFETY: The caller must uphold the all safety rules.
        Ok(unsafe { *p })
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
    // SAFETY: The caller must uphold the all safety notes.
    unsafe { try_as_mut(p).expect(ERR_MSG) }
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
        // SAFETY: The caller must uphold the all safety rules.
        Ok(unsafe { &mut *p })
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
    // SAFETY: The caller must uphold the all safety rules.
    unsafe { try_as_ref(p).expect(ERR_MSG) }
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
        // SAFETY: The caller must uphold the all safety rules.
        Ok(unsafe { &*p })
    } else {
        Err(Error::NotAligned)
    }
}

/// Casts a mutable pointer to another type of pointer.
///
/// # Panics
///
/// This method panics if the pointer after the cast is not aligned correctly.
pub fn cast_mut<T, U>(p: *mut T) -> *mut U {
    try_cast_mut(p).expect("The pointer is not aligned correctly.")
}

/// Casts a mutable pointer to another type of pointer.
///
/// # Errors
///
/// This method may return an [`Error::NotAligned`] error if the pointer after the cast is not
/// aligned correctly.
pub fn try_cast_mut<T, U>(p: *mut T) -> Result<*mut U, Error> {
    let after = p.cast::<U>();
    if is_aligned(after) {
        Ok(after)
    } else {
        Err(Error::NotAligned)
    }
}
