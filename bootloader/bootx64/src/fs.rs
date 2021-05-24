use core::convert::TryInto;
use core::slice;
use uefi_wrapper::protocols::media;

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn locate(path: &str) -> &[u8] {
    let mut st = crate::system_table();
    let bs = st.boot_services();

    let fs = bs.locate_protocol_without_registration::<media::SimpleFileSystem>();
    let mut fs = fs.expect("Failed to locate the Simple File System protoco.");

    let fp = fs.protocol.open_volume();
    let mut fp = fp.expect("Failed to open the root directory.");

    let r = fp.open_read_only(path);
    r.expect("Failed to open the file.");

    let size = filesize(&mut fp);
    let size: usize = size.try_into().unwrap();

    let buf = fs.bs.allocate_pool(size);
    let buf = buf.expect("Failed to allocate memory.");

    let buf = unsafe { slice::from_raw_parts_mut(buf, size) };

    let r = fp.read(buf);
    r.expect("Failed to read from the file.");

    buf
}

fn filesize(f: &mut media::File<'_>) -> u64 {
    const END_OF_FILE: u64 = !0;

    let r = f.set_position(END_OF_FILE);
    r.expect("Failed to set a file position.");

    let sz = f.get_position();
    let sz = sz.expect("Failed to get the filesize.");

    let r = f.set_position(0);
    r.expect("Failed to set a file position.");

    sz
}
