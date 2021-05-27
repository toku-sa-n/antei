use crate::result;
use aligned::ptr;
use core::ffi;
use core::fmt;
use core::mem;
use r_efi::efi;

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
    pub fn get_memory_map_size(&self) -> crate::Result<usize> {
        const SIZE: usize = 1;

        let mut sz = SIZE;
        let mut buf: mem::MaybeUninit<efi::MemoryDescriptor> = mem::MaybeUninit::uninit();
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
