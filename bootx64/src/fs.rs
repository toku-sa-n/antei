use core::convert::TryInto;
use core::slice;
use uefi::protocols::media;
use uefi::service;

#[must_use]
pub fn locate<'a>(st: &mut crate::SystemTable, path: &str) -> &'a [u8] {
    let r = try_locate(st, path);
    st.expect_ok(r, "Failed to locate a file.")
}

fn try_locate<'a>(st: &mut crate::SystemTable, path: &str) -> uefi::Result<&'a [u8]> {
    let bs = st.boot_services();

    let mut fs = bs.locate_protocol_without_registration::<media::SimpleFileSystem>()?;

    let mut fp = fs.protocol.open_volume()?;

    fp.open_read_only(path)?;

    let buf = try_allocate(&mut fp, &mut fs.bs)?;

    fp.read(buf).map_err(|e| e.map_value(|_| ()))?;

    Ok(buf)
}

fn try_allocate<'a, 'b>(
    f: &'a mut media::File<'_>,
    bs: &'a mut service::Boot<'_>,
) -> uefi::Result<&'b mut [u8]> {
    let sz = try_get_filesize(f)?;
    let sz: usize = sz.try_into().unwrap();

    let buf = bs.allocate_pool(sz)?;

    // SAFETY: `sz` bytes from the address `buf` are allocated by `allocate_pool` and it is a
    // single allocated object. These bytes are readable and writable. The memory is only acessible
    // from `buf`. The length must not be larger than `isize::MAX`.
    Ok(unsafe { slice::from_raw_parts_mut(buf, sz) })
}

fn try_get_filesize(f: &mut media::File<'_>) -> uefi::Result<u64> {
    const END_OF_FILE: u64 = !0;

    f.set_position(END_OF_FILE)?;

    let sz = f.get_position()?;

    f.set_position(0)?;

    Ok(sz)
}
