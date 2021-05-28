use core::mem;
use core::slice;
use uefi_wrapper::service::boot;

fn get_memory_map<'a>() -> (&'a [boot::MemoryDescriptor], boot::MapKey) {
    let with_paddings = allocate_buf_for_memory_map();
    let without_paddings = allocate_buf_for_memory_map();

    let mut st = crate::system_table();
    let mut bs = st.boot_services();

    let s = bs.get_memory_map(with_paddings);
    let (key, descriptors) = s.expect("Failed to fetch a memory map.");
    let num_descriptors = descriptors.len();

    for (i, d) in descriptors.enumerate() {
        let p =
            without_paddings.as_mut_ptr() as usize + mem::size_of::<boot::MemoryDescriptor>() * i;
        let p = p as *mut boot::MemoryDescriptor;

        // SAFETY: The memory is
        unsafe { aligned_ptr::write(p, d) };
    }

    // Can't use `slice::from_raw_parts` for `without_paddings` as the memory is referred by
    // `without_paddings`.

    todo!()
}

fn allocate_buf_for_memory_map<'a>() -> &'a mut [u8] {
    let sz = get_memory_map_size();
    let alloc_size = sz * 2; // `* 2` for the potential changes of the memory map.

    let mut st = crate::system_table();

    let mut bs = st.boot_services();

    let buf = bs.allocate_pool(alloc_size);
    let buf = buf.expect("Failed to allocate memory for the memory map.");

    // SAFETY: The memory is allocated by `allocate_pool`.
    unsafe { slice::from_raw_parts_mut(buf, alloc_size) }
}

fn get_memory_map_size() -> usize {
    let mut st = crate::system_table();
    let bs = st.boot_services();
    let sz = bs.get_memory_map_size();
    sz.expect("Failed to get the size of memory map.")
}
