use core::convert::TryInto;
use core::slice;
use uefi_wrapper::protocols::media;

pub fn locate(path: &str) -> &[u8] {
    let mut st = crate::system_table();
    let bs = st.boot_services();

    let fs = bs.locate_protocol_without_registration::<media::SimpleFileSystem>();
    let mut fs = fs.expect("Failed to locate the Simple File System protoco.");

    let fp = fs.protocol.open_volume();
    let mut fp = fp.expect("Failed to open the root directory.");

    let r = fp.open_read_only(path);
    r.expect("Failed to open the file.");

    let r = fp.set_position(!0);
    r.expect("Failed to set position.");

    let size = fp.get_position();
    let size = size.expect("Failed to get position.");
    let size: usize = size.try_into().unwrap();

    let r = fp.set_position(0);
    r.expect("Failed to set position.");

    let buf = fs.bs.allocate_pool(size);
    let buf = buf.expect("Failed to allocate memory.");

    let buf = unsafe { slice::from_raw_parts_mut(buf, size) };

    let r = fp.read(buf);
    r.expect("Failed to read from the file.");

    buf
}
