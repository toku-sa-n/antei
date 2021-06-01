pub fn get_memory_map_size(st: &mut crate::SystemTable) -> usize {
    let bs = st.boot_services();

    let size = bs.get_memory_map_size();
    size.expect("Failed to get the size of memory map.")
}
