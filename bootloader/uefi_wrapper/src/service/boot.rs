use crate::protocol::Protocol;
use crate::result;
use crate::result::Result;
use crate::system_table::SystemTable;
use core::fmt;
use core::mem::MaybeUninit;
use core::ptr;
use r_efi::efi;

pub struct Boot<'a>(&'a mut efi::BootServices, &'a mut SystemTable);
impl<'a> Boot<'a> {
    /// To avoid to create multiple pointers to the same protocol (which is potentially dangerous
    /// because it may create multiple mutable references to the same object), this method
    /// generates [`WithProtocol`].
    ///
    /// # Errors
    ///
    /// This method may return an `Err` value if the protocol is not found.
    pub fn locate_protocol_without_registration<P: Protocol>(self) -> Result<WithProtocol<'a, P>> {
        let mut protocol = MaybeUninit::uninit();
        let mut g = P::GUID;
        let s = (self.0.locate_protocol)(&mut g, ptr::null_mut(), protocol.as_mut_ptr());

        result::from_closure_and_status(
            move || {
                let protocol = unsafe { protocol.assume_init() }.cast::<P>();
                let protocol = unsafe { &mut *protocol };
                WithProtocol::new(protocol, self)
            },
            s,
        )
    }
}
impl<'a> From<&'a mut SystemTable> for Boot<'a> {
    fn from(s: &'a mut SystemTable) -> Self {
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
impl fmt::Debug for Boot<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Boot").finish()
    }
}

#[derive(Debug)]
pub struct WithProtocol<'a, P: Protocol> {
    pub protocol: &'a mut P,
    pub bs: Boot<'a>,
}
impl<'a, P: Protocol> WithProtocol<'a, P> {
    fn new(protocol: &'a mut P, bs: Boot<'a>) -> Self {
        Self { protocol, bs }
    }
}
