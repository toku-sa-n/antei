use {super::Mapper, accessor::marker, core::convert::TryInto, x86_64::PhysAddr};

type Generic<T, A> = accessor::single::Generic<T, Mapper, A>;

pub type ReadWrite<T> = Generic<T, marker::ReadWrite>;
pub type ReadOnly<T> = Generic<T, marker::ReadOnly>;
pub type WriteOnly<T> = Generic<T, marker::WriteOnly>;

/// # Safety
///
/// Refer to [`accessor::single::ReadWrite::new`].
#[cfg_attr(target_pointer_width = "64", allow(clippy::missing_panics_doc))]
#[must_use]
pub unsafe fn read_write<T>(p: PhysAddr) -> ReadWrite<T> {
    // SAFETY: The caller must uphold the safety requirements.
    unsafe { ReadWrite::new(p.as_u64().try_into().unwrap(), Mapper) }
}

/// # Safety
///
/// Refer to [`accessor::single::ReadOnly::new`].
#[cfg_attr(target_pointer_width = "64", allow(clippy::missing_panics_doc))]
#[must_use]
pub unsafe fn read_only<T>(p: PhysAddr) -> ReadOnly<T> {
    // SAFETY: The caller must uphold the safety requirements.
    unsafe { ReadOnly::new(p.as_u64().try_into().unwrap(), Mapper) }
}

/// # Safety
///
/// Refer to [`accessor::single::WriteOnly::new`].
#[cfg_attr(target_pointer_width = "64", allow(clippy::missing_panics_doc))]
#[must_use]
pub unsafe fn write_only<T>(p: PhysAddr) -> WriteOnly<T> {
    // SAFETY: The caller must uphold the safety requirements.
    unsafe { WriteOnly::new(p.as_u64().try_into().unwrap(), Mapper) }
}
