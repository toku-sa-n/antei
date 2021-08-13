use {
    crate::{error_unless, wrapping_sum_of_first_n_bytes, Error, Result, Xsdt},
    accessor::{single::ReadOnly, Mapper},
    core::convert::TryInto,
    x86_64::PhysAddr,
};

#[derive(Debug)]
pub struct Rsdp<M: Mapper>(ReadOnly<r_acpi::Rsdp, M>);
impl<M: Mapper> Rsdp<M> {
    /// # Safety
    ///
    /// `a` must be the correct address of RSDP.
    ///
    /// # Errors
    ///
    /// This method returns an error if the fetched RSDP is invalid (wrong signature, etc.).
    #[cfg_attr(target_pointer_width = "64", allow(clippy::missing_panics_doc))]
    pub unsafe fn from_addr(a: PhysAddr, m: M) -> Result<Self> {
        // SAFETY: The caller must ensure that `a` is the correct address of RSDP.
        let accessor =
            unsafe { ReadOnly::<r_acpi::Rsdp, _>::new(a.as_u64().try_into().unwrap(), m) };

        Validator(accessor.read_volatile()).validate()?;

        Ok(Self(accessor))
    }

    /// # Errors
    ///
    /// This method returns an error if the fetched XSDT is corrupt (e.g., wrong signature,
    /// checksum, etc.).
    pub fn xsdt<M1: Mapper + Clone>(&self, m: M1) -> Result<Xsdt<M1>> {
        let base = self.0.read_volatile().xsdt_address;
        let base = PhysAddr::new(base);

        // SAFETY: The address is correct.
        unsafe { Xsdt::new(base, m) }
    }
}

struct Validator(r_acpi::Rsdp);
impl Validator {
    fn validate(self) -> Result<()> {
        self.validate_signature()?;
        self.validate_revision()?;
        self.validate_checksum_20bytes()?;
        self.validate_checksum_entire_bytes()?;

        Ok(())
    }

    fn validate_signature(&self) -> Result<()> {
        error_unless(&self.0.signature == b"RSD PTR ", Error::RsdpWrongSignature)
    }

    fn validate_revision(&self) -> Result<()> {
        error_unless(self.0.revision == 2, Error::RsdpUnsupportedRevision)
    }

    fn validate_checksum_20bytes(&self) -> Result<()> {
        error_unless(
            wrapping_sum_of_first_n_bytes(&self.0, 20) == 0,
            Error::RsdpWrongChecksumOfFirst20Bytes,
        )
    }

    fn validate_checksum_entire_bytes(&self) -> Result<()> {
        error_unless(
            wrapping_sum_of_first_n_bytes(&self.0, self.0.length.try_into().unwrap()) == 0,
            Error::RsdpWrongChecksumOfFullBytes,
        )
    }
}
