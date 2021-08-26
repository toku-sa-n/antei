use {
    aligned_ptr::slice, core::convert::TryInto, os_units::NumOfPages,
    x86_64::structures::paging::Size4KiB,
};

pub(super) fn fetch<'a>(name: &str) -> Option<&'a [u8]> {
    for entry in cpio_reader::iter_files(initrd_binary()) {
        if entry.name() == name {
            return Some(entry.file());
        }
    }

    None
}

fn initrd_binary<'a>() -> &'a [u8] {
    use predefined_mmap::initrd;

    let num_of_pages = initrd().end - initrd().start;
    let num_of_pages = NumOfPages::<Size4KiB>::new(num_of_pages.try_into().unwrap());

    let start = initrd().start.start_address().as_ptr();

    unsafe { slice::from_raw_parts(start, num_of_pages.as_bytes().as_usize()) }
}
