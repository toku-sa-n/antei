use crate::result;
use aligned_ptr::ptr;
use core::ffi;
use core::fmt;
use core::mem;
use r_efi::efi;

pub use r_efi::efi::MemoryDescriptor;

pub struct Boot<'a> {
    bs: &'a mut efi::BootServices,
    _st: &'a mut crate::SystemTable,
}
impl<'a> Boot<'a> {
    /// To avoid to create multiple pointers to the same protocol (which is potentially dangerous
    /// as it may create multiple mutable references to the same object), this method generates
    /// [`WithProtocol`].
    ///
    /// # Errors
    ///
    /// Refer to the UEFI specification.
    pub fn locate_protocol_without_registration<P: crate::Protocol>(
        self,
    ) -> crate::Result<WithProtocol<'a, P>> {
        const WITHOUT_REGISTRATION: *mut ffi::c_void = core::ptr::null_mut();

        let mut protocol = mem::MaybeUninit::uninit();
        let mut g = P::GUID;
        let r = (self.bs.locate_protocol)(&mut g, WITHOUT_REGISTRATION, protocol.as_mut_ptr());

        result::from_status_and_closure(r, || {
            // SAFETY: `locate_protocol` initializes `protocol`.
            let protocol = unsafe { protocol.assume_init() }.cast::<P>();

            // SAFETY: On success, `protocol` is initialized to the pointer to the protocol.
            //
            // There is no mutable references to the protocol as there is no way to create it
            // without this method.
            let protocol = unsafe { ptr::as_mut(protocol) };
            WithProtocol::new(protocol, self)
        })
    }

    /// # Errors
    ///
    /// Refer to the UEFI specification.
    pub fn allocate_pool(&mut self, size: usize) -> crate::Result<*mut u8> {
        const MEMORY_TYPE: efi::MemoryType = efi::MemoryType::LoaderData;
        let mut buf = mem::MaybeUninit::uninit();
        let r = (self.bs.allocate_pool)(MEMORY_TYPE, size, buf.as_mut_ptr());

        result::from_status_and_closure(r, || {
            // SAFETY: `allocate_pool` initializes `buf`.
            unsafe { buf.assume_init() }.cast()
        })
    }

    /// # Errors
    ///
    /// Refer to the UEFI specification.
    pub fn free_pool(&mut self, buf: *mut u8) -> crate::Result<()> {
        let r = (self.bs.free_pool)(buf.cast());
        result::from_status_and_value(r, ())
    }

    /// # Errors
    ///
    /// Refer to the UEFI specification.
    pub fn get_memory_map<'b>(
        &self,
        buf: &'b mut [u8],
    ) -> crate::Result<(MapKey, impl ExactSizeIterator<Item = MemoryDescriptor> + 'b), Option<usize>>
    {
        let mut memory_map_size = buf.len();
        let mut map_key = mem::MaybeUninit::uninit();
        let mut descriptor_size = mem::MaybeUninit::uninit();
        let mut descriptor_version = mem::MaybeUninit::uninit();

        let s = (self.bs.get_memory_map)(
            &mut memory_map_size,
            buf.as_mut_ptr().cast(),
            map_key.as_mut_ptr(),
            descriptor_size.as_mut_ptr(),
            descriptor_version.as_mut_ptr(),
        );

        if s == efi::Status::SUCCESS {
            // SAFETY: `get_memory_map` initializes `map_key`.
            let map_key = MapKey(unsafe { map_key.assume_init() });

            // SAFETY: `get_memory_map` initializes `descriptor_size`.
            let descriptor_size = unsafe { descriptor_size.assume_init() };

            // SAFETY: `buf.as_ptr()` points to the first memory descriptor.
            Ok((map_key, unsafe {
                MemoryMapIter::new(buf, memory_map_size, descriptor_size)
            }))
        } else {
            Err(crate::Error::from_status_and_value(
                s.into(),
                if s == efi::Status::BUFFER_TOO_SMALL {
                    Some(memory_map_size)
                } else {
                    None
                },
            ))
        }
    }

    /// # Errors
    ///
    /// Refer to the UEFI specification.
    pub fn get_memory_map_size(&self) -> crate::Result<usize> {
        const SIZE: usize = 1;

        let mut sz = SIZE;
        let mut buf: mem::MaybeUninit<MemoryDescriptor> = mem::MaybeUninit::uninit();
        let mut map_key = mem::MaybeUninit::uninit();
        let mut descriptor_size = mem::MaybeUninit::uninit();
        let mut descriptor_version = mem::MaybeUninit::uninit();

        let r = (self.bs.get_memory_map)(
            &mut sz,
            buf.as_mut_ptr(),
            map_key.as_mut_ptr(),
            descriptor_size.as_mut_ptr(),
            descriptor_version.as_mut_ptr(),
        );

        match r {
            efi::Status::BUFFER_TOO_SMALL => Ok(sz),
            efi::Status::SUCCESS => unreachable!(),
            _ => Err(r.into()),
        }
    }

    pub fn new(bs: &'a mut efi::BootServices, st: &'a mut crate::SystemTable) -> Self {
        Self { bs, _st: st }
    }
}
impl<'a, P: crate::Protocol> From<WithProtocol<'a, P>> for Boot<'a> {
    fn from(w: WithProtocol<'a, P>) -> Self {
        w.bs
    }
}
impl fmt::Debug for Boot<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Boot").finish()
    }
}

#[derive(Debug)]
pub struct WithProtocol<'a, P: crate::Protocol> {
    pub protocol: &'a mut P,
    pub bs: Boot<'a>,
}
impl<'a, P: crate::Protocol> WithProtocol<'a, P> {
    fn new(protocol: &'a mut P, bs: Boot<'a>) -> Self {
        Self { protocol, bs }
    }
}

struct MemoryMapIter<'a> {
    buf: &'a [u8],
    descriptor_size: usize,
    i: usize,
    len: usize,
}
impl<'a> MemoryMapIter<'a> {
    /// # Safety
    ///
    /// `buf.as_ptr()` must point to a memory descriptor.
    unsafe fn new(buf: &'a [u8], memory_map_size: usize, descriptor_size: usize) -> Self {
        Self {
            buf,
            descriptor_size,
            i: 0,
            len: memory_map_size / descriptor_size,
        }
    }
}
impl Iterator for MemoryMapIter<'_> {
    type Item = MemoryDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.len {
            let p = self.buf.as_ptr() as usize + self.descriptor_size * self.i;
            let p = p as *const MemoryDescriptor;

            let d = unsafe { p.read_unaligned() };

            self.i += 1;

            Some(d)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}
impl ExactSizeIterator for MemoryMapIter<'_> {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapKey(usize);
impl From<MapKey> for usize {
    fn from(k: MapKey) -> Self {
        k.0
    }
}
