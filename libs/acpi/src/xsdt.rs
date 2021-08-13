use {
    crate::{error_unless, wrapping_sum_of_bytes, Error, Fadt, Result},
    accessor::{single, Mapper},
    core::{
        convert::{TryFrom, TryInto},
        mem::size_of,
    },
    x86_64::PhysAddr,
};

#[derive(Debug)]
pub struct Xsdt<M>
where
    M: Mapper + Clone,
{
    pub header: Header<M>,
    pub entry: Entries<M>,
}
impl<M> Xsdt<M>
where
    M: Mapper + Clone,
{
    /// # Safety
    ///
    /// `base` must be the correct address of XSDT.
    ///
    /// # Errors
    ///
    /// This method returns an error if the XSDT is broken (e.g., wrong signature, checksum, etc.).
    pub unsafe fn new(base: PhysAddr, mapper: M) -> Result<Self> {
        // SAFETY: The caller must ensure that `base` is the start address of XSDT.
        let header = unsafe { Header::new(base, mapper.clone())? };

        // SAFETY: The caller must ensure that `base` is the start address of XSDT.
        let entry =
            unsafe { Entries::new(base + size_of::<r_acpi::Xsdt>(), header.entry_len(), mapper) };

        validate_checksum(&header.0.read_volatile(), entry.into_iter())?;

        Ok(Self { header, entry })
    }

    /// # Errors
    ///
    /// This method returns an error if FADT is broken (e.g., wrong checksum, etc.).
    pub fn fadt<M2: Mapper + Clone>(&self, m: &M2) -> Result<Option<Fadt<M2>>> {
        self.entry
            .into_iter()
            .filter(|a| {
                let _ = &m;

                // SAFETY: The first 4 bytes of the all system description tables are always readable.
                unsafe { is_fadt(*a, m.clone()) }
            })
            .find_map(|a| {
                let _ = &m;

                // SAFETY: `a` is the address of FADT.
                match unsafe { Fadt::new(a, m.clone()) } {
                    Err(Error::FadtWrongSignature) => unreachable!(),
                    a => Some(a),
                }
            })
            .transpose()
    }
}

#[derive(Debug)]
pub struct Header<M: Mapper>(single::ReadOnly<r_acpi::Xsdt, M>);
impl<M: Mapper> Header<M> {
    /// # Safety
    ///
    /// `base` must be the address of the header of XSDT.
    ///
    /// # Errors
    ///
    /// This method returns an error if the XSDT is broken (e.g., wrong signature, checksum, etc.).
    #[cfg_attr(target_pointer_width = "64", allow(clippy::missing_panics_doc))]
    pub unsafe fn new(base: PhysAddr, mapper: M) -> Result<Self> {
        // SAFETY: The caller must ensure that `base` is the address of the header of XSDT.
        let accessor = unsafe { single::ReadOnly::new(base.as_u64().try_into().unwrap(), mapper) };

        HeaderValidator(accessor.read_volatile()).validate()?;

        Ok(Self(accessor))
    }

    fn entry_len(&self) -> usize {
        let entry_size: usize = usize::try_from(self.0.read_volatile().header.length).unwrap()
            - size_of::<r_acpi::Xsdt>();

        assert_eq!(entry_size % size_of::<PhysAddr>(), 0, "Invalid length");

        entry_size / size_of::<PhysAddr>()
    }
}

struct HeaderValidator(r_acpi::Xsdt);
impl HeaderValidator {
    fn validate(self) -> Result<()> {
        self.validate_signature()?;
        self.validate_revision()?;
        self.validate_length()?;

        Ok(())
    }

    fn validate_signature(&self) -> Result<()> {
        error_unless(
            &self.0.header.signature == b"XSDT",
            Error::XsdtWrongSignature,
        )
    }

    fn validate_revision(&self) -> Result<()> {
        error_unless(self.0.header.revision == 1, Error::XsdtWrongRevision)
    }

    fn validate_length(&self) -> Result<()> {
        let entry_size = usize::try_from(self.0.header.length).unwrap() - size_of::<r_acpi::Xsdt>();

        error_unless(
            entry_size % size_of::<PhysAddr>() == 0,
            Error::XsdtInvalidEntryLength,
        )
    }
}

#[derive(Debug)]
pub struct Entries<M: Mapper + Clone> {
    base: PhysAddr,
    len: usize,
    mapper: M,
}
impl<M: Mapper + Clone> Entries<M> {
    /// # Safety
    ///
    /// - `base` must be the start address of the entries of XSDT.
    /// - `len` must be the correct number of addresses.
    #[cfg_attr(target_pointer_width = "64", allow(clippy::missing_panics_doc))]
    pub unsafe fn new(base: PhysAddr, len: usize, mapper: M) -> Self {
        Self { base, len, mapper }
    }

    // ACPI specification does not tell the access width of each table. However, `read_volatile`
    // requires a well-aligned address, so we cannot use that function directly with a pointer of
    // `PhysAddr` because the entries start from 36 bytes offset from the start of the header, so
    // it may be misaligned. This method firstly gets the address as a byte array, then converts
    // the byte array into a `u64` value to avoid the alignment issue.
    fn at(&self, index: usize) -> PhysAddr {
        assert!(index < self.len, "Index out of range.");

        let addr = self.base + index * size_of::<PhysAddr>();

        // SAFETY: `Self::new` ensures that `base` is the correct start address. `index` is in the
        // range because `Self::new` ensures that `len` is the correct number of entries.
        let accessor = unsafe {
            single::ReadOnly::new(addr.as_u64().try_into().unwrap(), self.mapper.clone())
        };

        let bytes: [u8; 8] = accessor.read_volatile();

        let addr = u64::from_le_bytes(bytes);

        PhysAddr::new(addr)
    }
}
impl<'a, M: Mapper + Clone> IntoIterator for &'a Entries<M> {
    type Item = PhysAddr;
    type IntoIter = Iter<'a, M>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

#[derive(Debug)]
pub struct Iter<'a, M: Mapper + Clone> {
    entries: &'a Entries<M>,
    index: usize,
}
impl<'a, M: Mapper + Clone> Iter<'a, M> {
    fn new(entries: &'a Entries<M>) -> Self {
        Self { entries, index: 0 }
    }
}
impl<M: Mapper + Clone> Iterator for Iter<'_, M> {
    type Item = PhysAddr;

    fn next(&mut self) -> Option<Self::Item> {
        (self.index < self.entries.len).then(|| {
            let _ = &self;

            let addr = self.entries.at(self.index);

            self.index += 1;

            addr
        })
    }
}

fn validate_checksum(h: &r_acpi::Xsdt, entry: impl Iterator<Item = PhysAddr>) -> Result<()> {
    error_unless(calculate_checksum(h, entry) == 0, Error::XsdtWrongChecksum)
}

fn calculate_checksum(h: &r_acpi::Xsdt, entry: impl Iterator<Item = PhysAddr>) -> u8 {
    wrapping_sum_of_bytes(h).wrapping_add(
        entry
            .map(|a| wrapping_sum_of_bytes(&a))
            .fold(0_u8, u8::wrapping_add),
    )
}

/// # Safety
///
/// 4 bytes from `base` must be readable.
unsafe fn is_fadt<M2: Mapper>(base: PhysAddr, m: M2) -> bool {
    // SAFETY: The caller must ensure that 4 bytes from `base` are readable.
    let signature =
        unsafe { single::ReadOnly::<[u8; 4], _>::new(base.as_u64().try_into().unwrap(), m) };

    &signature.read_volatile() == b"FACP"
}
