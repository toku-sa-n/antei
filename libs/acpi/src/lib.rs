#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

use {accessor::Mapper, core::mem::size_of, x86_64::PhysAddr};

pub use {
    error::Error, fadt::Fadt, generic_address_structure::GenericAddressStructure, rsdp::Rsdp,
    xsdt::Xsdt,
};

pub type Result<T> = core::result::Result<T, Error>;

mod error;
pub mod fadt;
mod generic_address_structure;
mod rsdp;
pub mod xsdt;

#[derive(Debug)]
pub struct Tables<M: Mapper + Clone> {
    pub rsdp: Rsdp<M>,
    pub xsdt: Xsdt<M>,
    pub fadt: Option<Fadt<M>>,
}
impl<M: Mapper + Clone> Tables<M> {
    /// # Safety
    ///
    /// `a` must be the correct address of RSDP.
    ///
    /// # Errors
    ///
    /// This method returns an error if one of the tables is corrupt (e.g., wrong signature, checksum, etc.).
    pub unsafe fn from_rsdp_addr(a: PhysAddr, m: &M) -> Result<Self> {
        // SAFETY: The caller must ensure that `a` is the correct address of RSDP.
        let rsdp = unsafe { Rsdp::from_addr(a, m.clone()) }?;
        let xsdt = rsdp.xsdt(m.clone())?;
        let fadt = xsdt.fadt(m)?;

        Ok(Self { rsdp, xsdt, fadt })
    }
}

fn wrapping_sum_of_bytes<T>(v: &T) -> u8 {
    wrapping_sum_of_first_n_bytes(v, size_of::<T>())
}

fn wrapping_sum_of_first_n_bytes<T>(v: &T, n: usize) -> u8 {
    assert!(
        n <= size_of::<T>(),
        "n must be less or equal to size_of::<T>()"
    );

    // SAFETY: The pointer points to the address that is in the range of `v`.
    (0..n).fold(0, |acc, x| unsafe {
        let _ = &v;

        acc.wrapping_add({
            let v: *const _ = v;
            v.cast::<u8>().add(x).read()
        })
    })
}

fn error_unless(cond: bool, e: Error) -> Result<()> {
    if cond {
        Ok(())
    } else {
        Err(e)
    }
}
