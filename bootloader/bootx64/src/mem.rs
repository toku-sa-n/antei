pub fn get_memory_map_size() -> usize {
    let mut st = crate::system_table();

    let bs = st.boot_services();

    let size = bs.get_memory_map_size();
    size.expect("Failed to get the size of memory map.")
}
