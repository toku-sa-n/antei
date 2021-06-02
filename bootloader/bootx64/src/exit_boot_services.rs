use aligned::ptr;
use aligned::slice;
use uefi_wrapper::service::boot;

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

    let mmap_size = bs.get_memory_map_size()?;

    let alloc_size_for_mmap = mmap_size * 2;

    let raw_mmap_ptr = bs.allocate_pool(alloc_size_for_mmap)?;

    let descriptor_array_ptr = bs.allocate_pool(alloc_size_for_mmap)?;
    let descriptor_array_ptr = ptr::cast_mut::<_, boot::MemoryDescriptor>(descriptor_array_ptr);

    // SAFETY: `alloc_size_for_mmap` bytes from `raw_mmap_ptr` are allocated by `allocate_pool`.
    // These memory are readable, writable, and byte-aligned.
    //
    // `raw_mmap_ptr` must not be used from this line.
    let mut raw_mmap_buf = unsafe { slice::from_raw_parts_mut(raw_mmap_ptr, alloc_size_for_mmap) };

    let (key, descriptor_iter) = bs
        .get_memory_map(&mut raw_mmap_buf)
        .map_err(|e| e.map_value(|_| ()))?;

    st.exit_boot_services(h, key)
        .map_err(|e| e.map_value(|_| ()))?;

    let mmap_len = descriptor_iter.len();

    for (i, d) in descriptor_iter.enumerate() {
        // SAFETY: `p` points to an address which is allocated by `allocate_pool`.
        unsafe {
            let p = descriptor_array_ptr.add(i);
            ptr::write(p, d);
        }
    }

    // SAFETY: `mmap_len` bytes from `mmap_array_ptr` are in the range of memory allocated by
    // `allocate_pool.` These memory are initialized by the `for` statement.
    //
    // `mmap_array_ptr` must not be used from this line.
    let descriptors = unsafe { slice::from_raw_parts_mut(descriptor_array_ptr, mmap_len) };

    Ok(descriptors)
}
