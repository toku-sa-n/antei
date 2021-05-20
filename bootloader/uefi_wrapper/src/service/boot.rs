use crate::result;
use core::ffi;
use core::fmt;
use core::mem;
use core::ptr;
use r_efi::efi;

pub struct Boot<'a>(&'a mut efi::BootServices, &'a mut crate::SystemTable);
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
        let mut protocol = mem::MaybeUninit::uninit();
        let mut g = P::GUID;
        let r = (self.0.locate_protocol)(&mut g, ptr::null_mut(), protocol.as_mut_ptr());

        result::from_status_and_closure(r, || {
            // SAFETY: `locate_protocol` initializes `protocol`.
            let protocol = unsafe { protocol.assume_init() }.cast::<P>();

            // SAFETY: On success, `protocol` is initialized to the pointer to the protocol.
            //
            // There is no mutable references to the protocol as there is no way to create it
            // without this method.
            let protocol = unsafe { &mut *protocol };
            WithProtocol::new(protocol, self)
        })
    }

    /// # Errors
    ///
    /// Refer to the UEFI specification.
    pub fn allocate_pool(&mut self, size: usize) -> crate::Result<*mut u8> {
        const MEMORY_TYPE: efi::MemoryType = efi::MemoryType::ConventionalMemory;
        let mut buf = mem::MaybeUninit::uninit();
        let r = (self.0.allocate_pool)(MEMORY_TYPE, size, buf.as_mut_ptr());

        result::from_status_and_closure(r, || {
            // SAFETY: `allocate_pool` initializes `buf`.
            unsafe { buf.assume_init() }.cast()
        })
    }

    /// # Errors
    ///
    /// Refer to the UEFI specification.
    pub fn free_pool(&mut self, buf: *mut ffi::c_void) -> crate::Result<()> {
        let r = (self.0.free_pool)(buf);
        result::from_status_and_value(r, ())
    }
}
impl<'a> From<&'a mut crate::SystemTable> for Boot<'a> {
    fn from(s: &'a mut crate::SystemTable) -> Self {
        let s_ptr = s.get_ptr();

        // SAFETY: `SystemTable` is created only from the argument of `efi_main`. We must trust the
        // argument is a valid pointer.
        //
        // There exists only one `SystemTable`, so do `Boot`. This is why the mutable reference is
        // only one it exists.
        let bs = unsafe { &mut *(*s_ptr).boot_services };

        Self(bs, s)
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
