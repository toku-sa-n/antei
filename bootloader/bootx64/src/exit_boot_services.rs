use aligned::ptr;
use aligned::slice;
use uefi_wrapper::service::{self, boot};

#[must_use]
pub fn exit_boot_services<'a>(
    h: uefi_wrapper::Handle,
    st: crate::SystemTable,
) -> &'a mut [boot::MemoryDescriptor] {
    try_exit_boot_services(h, st).expect("Failed to exit boot services.")
}

fn try_exit_boot_services<'a>(
    h: uefi_wrapper::Handle,
    mut st: crate::SystemTable,
) -> uefi_wrapper::Result<&'a mut [boot::MemoryDescriptor]> {
    let mut bs = st.boot_services();

    let mut raw_mmap_buf = try_alloc_for_raw_mmap(&mut bs)?;

    let descriptor_array_ptr = try_alloc_for_descriptors_array(&mut bs)?;

    let (key, descriptor_iter) = bs
        .get_memory_map(&mut raw_mmap_buf)
        .map_err(|e| e.map_value(|_| ()))?;

    st.exit_boot_services(h, key)
        .map_err(|e| e.map_value(|_| ()))?;

    // SAFETY: `mmap_len` bytes from `mmap_array_ptr` are in the range of memory allocated by
    // `allocate_pool.` These memory are initialized by the `for` statement.
    //
    // `mmap_array_ptr` must not be used from this line.
    let descriptors = unsafe { generate_descriptors_array(descriptor_iter, descriptor_array_ptr) };

    Ok(descriptors)
}

fn try_alloc_for_raw_mmap<'a, 'b, 'c>(
    bs: &'a mut service::Boot<'b>,
) -> uefi_wrapper::Result<&'c mut [u8]> {
    let size = try_get_alloc_size_for_mmap(bs)?;
    let ptr = bs.allocate_pool(size)?;

    // SAFETY: `size` bytes from `ptr` are allocated by `allocate_pool`.
    // These memory are readable, writable, and byte-aligned.
    Ok(unsafe { slice::from_raw_parts_mut(ptr, size) })
}

fn try_alloc_for_descriptors_array(
    bs: &mut service::Boot<'_>,
) -> uefi_wrapper::Result<*mut boot::MemoryDescriptor> {
    let size = try_get_alloc_size_for_mmap(bs)?;
    bs.allocate_pool(size).map(ptr::cast_mut)
}

fn try_get_alloc_size_for_mmap(bs: &mut service::Boot<'_>) -> uefi_wrapper::Result<usize> {
    Ok(bs.get_memory_map_size()? * 2)
}

/// # Safety
///
/// The `size_of::<boot::MemoryDescriptor> * descriptors.len()` bytes from `array_ptr` must be
/// readable, writable, and dereferencable.
///
/// After calling this function, the caller must not derefer `array_ptr` unless the returned
/// reference is dropped.
unsafe fn generate_descriptors_array<'a>(
    descriptors: impl ExactSizeIterator<Item = boot::MemoryDescriptor>,
    array_ptr: *mut boot::MemoryDescriptor,
) -> &'a mut [boot::MemoryDescriptor] {
    let mmap_len = descriptors.len();

    for (i, d) in descriptors.enumerate() {
        let p = array_ptr.add(i);
        ptr::write(p, d);
    }

    // SAFETY: The caller must ensure that `mmap_len` bytes from `mmap_array_ptr` are dereferencable.
    // These memory are initialized by the `for` statement.
    slice::from_raw_parts_mut(array_ptr, mmap_len)
}
