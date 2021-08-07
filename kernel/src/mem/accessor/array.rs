use {super::Mapper, accessor::marker, core::convert::TryInto, x86_64::PhysAddr};

type Generic<T, A> = accessor::array::Generic<T, Mapper, A>;

pub(crate) type ReadWrite<T> = Generic<T, marker::ReadWrite>;
pub(crate) type ReadOnly<T> = Generic<T, marker::ReadOnly>;
pub(crate) type WriteOnly<T> = Generic<T, marker::WriteOnly>;

/// # Safety
///
/// Refer to [`accessor::single::ReadWrite::new`].
pub(crate) unsafe fn read_write<T: Copy>(p: PhysAddr, len: usize) -> ReadWrite<T> {
    // SAFETY: The caller must uphold the safety requirements.
    unsafe { ReadWrite::new(p.as_u64().try_into().unwrap(), len, Mapper) }
}

/// # Safety
///
/// Refer to [`accessor::single::ReadOnly::new`].
pub(crate) unsafe fn read_only<T: Copy>(p: PhysAddr, len: usize) -> ReadOnly<T> {
    // SAFETY: The caller must uphold the safety requirements.
    unsafe { ReadOnly::new(p.as_u64().try_into().unwrap(), len, Mapper) }
}

/// # Safety
///
/// Refer to [`accessor::single::WriteOnly::new`].
pub(crate) unsafe fn write_only<T: Copy>(p: PhysAddr, len: usize) -> WriteOnly<T> {
    // SAFETY: The caller must uphold the safety requirements.
    unsafe { WriteOnly::new(p.as_u64().try_into().unwrap(), len, Mapper) }
}
