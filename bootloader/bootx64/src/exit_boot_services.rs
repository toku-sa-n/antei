use crate::SystemTable;
use aligned::ptr;
use aligned::slice;
use boot::MemoryDescriptor;
use uefi_wrapper::{
    service::{self, boot},
    Handle, Result,
};

#[must_use]
#[allow(clippy::module_name_repetitions)]
pub fn exit_boot_services_and_return_mmap<'a>(
    h: Handle,
    st: SystemTable,
) -> &'a mut [MemoryDescriptor] {
    try_exit_boot_services_and_return_mmap(h, st).expect("Failed to exit boot services.")
}

fn try_exit_boot_services_and_return_mmap<'a>(
    h: Handle,
    mut st: SystemTable,
) -> Result<&'a mut [MemoryDescriptor]> {
    let mut bs = st.boot_services();

    let raw_mmap_buf = try_alloc_for_raw_mmap(&mut bs)?;

    let descriptor_array_ptr = try_alloc_for_descriptors_array(&mut bs)?;

    let descriptor_iter = try_exit_boot_services(h, st, raw_mmap_buf)?;

    // SAFETY: `descriptor_array_ptr` points to the memory allocated by
    // `try_alloc_for_descriptors_array`.
    Ok(unsafe { generate_descriptors_array(descriptor_iter, descriptor_array_ptr) })
}

fn try_alloc_for_raw_mmap<'a>(bs: &mut service::Boot<'_>) -> Result<&'a mut [u8]> {
    let size = try_get_alloc_size_for_mmap(bs)?;
    let ptr = bs.allocate_pool(size)?;

    // SAFETY: `size` bytes from `ptr` are allocated by `allocate_pool`.
    // These memory are readable, writable, and byte-aligned.
    Ok(unsafe { slice::from_raw_parts_mut(ptr, size) })
}

fn try_get_alloc_size_for_mmap(bs: &mut service::Boot<'_>) -> Result<usize> {
    bs.get_memory_map_size().map(|size| size * 2)
}

fn try_alloc_for_descriptors_array(bs: &mut service::Boot<'_>) -> Result<*mut MemoryDescriptor> {
    let size = try_get_alloc_size_for_mmap(bs)?;
    bs.allocate_pool(size).map(ptr::cast_mut)
}

fn try_exit_boot_services(
    h: Handle,
    mut st: SystemTable,
    mmap_buf: &mut [u8],
) -> Result<impl ExactSizeIterator<Item = MemoryDescriptor> + '_> {
    let bs = st.boot_services();

    let (key, descriptor_iter) = bs
        .get_memory_map(mmap_buf)
        .map_err(|e| e.map_value(|_| ()))?;

    st.exit_boot_services(h, key)
        .map_err(|e| e.map_value(|_| ()))?;

    Ok(descriptor_iter)
}

/// # Safety
///
/// The `size_of::<boot::MemoryDescriptor>() * descriptors.len()` bytes from `array_ptr` must be
/// readable, writable, and dereferencable.
///
/// After calling this function, the caller must not derefer `array_ptr` unless the returned
/// reference is dropped.
unsafe fn generate_descriptors_array<'a>(
    descriptors: impl ExactSizeIterator<Item = MemoryDescriptor>,
    array_ptr: *mut MemoryDescriptor,
) -> &'a mut [MemoryDescriptor] {
    let mmap_len = descriptors.len();

    for (i, d) in descriptors.enumerate() {
        let p = array_ptr.add(i);
        ptr::write(p, d);
    }

    // SAFETY: The caller must ensure that `mmap_len` bytes from `array_ptr` are dereferencable.
    // These memory are initialized by the `for` statement.
    slice::from_raw_parts_mut(array_ptr, mmap_len)
}
