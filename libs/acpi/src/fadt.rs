use {
    crate::{error_unless, wrapping_sum_of_first_n_bytes, Error, GenericAddressStructure, Result},
    accessor::{single::ReadOnly, Mapper},
    bit_field::BitField,
    core::convert::TryInto,
    x86_64::PhysAddr,
};

pub use r_acpi::PM_TIMER_FREQUENCY_HZ;

const NULL_GAS: r_acpi::GenericAddressStructure = r_acpi::GenericAddressStructure {
    access_size: 0,
    address: 0,
    address_space_id: 0,
    register_bit_offset: 0,
    register_bit_width: 0,
};

#[derive(Debug)]
pub struct Fadt<M: Mapper>(ReadOnly<r_acpi::Fadt, M>);
impl<M: Mapper> Fadt<M> {
    /// # Safety
    ///
    /// `base` must be the correct address of FADT.
    ///
    /// # Errors
    ///
    /// This method returns an error if FADT is broken (e.g. wrong signature, wrong checksum,
    /// etc.).
    #[cfg_attr(target_pointer_width = "64", allow(clippy::missing_panics_doc))]
    pub unsafe fn new(base: PhysAddr, mapper: M) -> Result<Self> {
        // SAFETY: The caller must ensure that `base` is the correct address of FADT.
        let fadt = unsafe { ReadOnly::new(base.as_u64().try_into().unwrap(), mapper) };

        Validator(fadt.read_volatile()).validate()?;

        Ok(Self(fadt))
    }

    pub fn pm_timer(&self) -> Option<Result<PmTimer>> {
        PmTimer::new(&self.0.read_volatile())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PmTimer {
    address: GenericAddressStructure,
    width: TimerRegisterWidth,
}
impl PmTimer {
    #[allow(clippy::if_not_else)]
    #[must_use]
    pub fn new(fadt: &r_acpi::Fadt) -> Option<Result<Self>> {
        Self::from_x_pm_tmr_blk(fadt).or_else(|| Self::from_pm_tmr_blk(fadt).map(Ok))
    }

    #[must_use]
    pub fn address(&self) -> GenericAddressStructure {
        self.address
    }

    #[must_use]
    pub fn width(&self) -> TimerRegisterWidth {
        self.width
    }

    fn from_x_pm_tmr_blk(fadt: &r_acpi::Fadt) -> Option<Result<Self>> {
        // SAFETY: `x_pm_tmr_blk_exists` ensures that `X_PM_TMR_BLK` exists.
        Self::x_pm_tmr_blk_exists(fadt).then(|| unsafe { Self::from_x_pm_tmr_blk_unchecked(fadt) })
    }

    fn from_pm_tmr_blk(fadt: &r_acpi::Fadt) -> Option<Self> {
        // SAFETY: `pm_tmr_blk_exists` ensures that `PM_TMR_BLK` exists.
        Self::pm_tmr_blk_exists(fadt).then(|| Self::from_pm_tmr_blk_unchecked(fadt))
    }

    /// # Safety
    ///
    /// This method is unsafe because it will create an invalid address if `X_PM_TMR_BLK` does not
    /// exist.
    ///
    /// The caller must ensure that the FADT's major version is more than 1.
    unsafe fn from_x_pm_tmr_blk_unchecked(fadt: &r_acpi::Fadt) -> Result<Self> {
        assert_ne!(fadt.x_pm_tmr_blk, NULL_GAS, "X_PM_TMR_BLK is 0.");

        fadt.x_pm_tmr_blk.try_into().map(|address| Self {
            address,
            width: fadt.into(),
        })
    }

    fn from_pm_tmr_blk_unchecked(fadt: &r_acpi::Fadt) -> Self {
        let pm_tmr_blk = fadt.pm_tmr_blk;

        assert_ne!(pm_tmr_blk, 0, "PM_TMR_BLK is 0.");

        Self {
            address: GenericAddressStructure::SystemIo(pm_tmr_blk.into()),
            width: fadt.into(),
        }
    }

    fn x_pm_tmr_blk_exists(fadt: &r_acpi::Fadt) -> bool {
        let major_version = fadt.header.revision;
        let blk = fadt.x_pm_tmr_blk;

        major_version >= 2 && blk != NULL_GAS
    }

    fn pm_tmr_blk_exists(fadt: &r_acpi::Fadt) -> bool {
        fadt.pm_tmr_blk != 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerRegisterWidth {
    Bits32,
    Bits24,
}
impl From<r_acpi::Fadt> for TimerRegisterWidth {
    fn from(f: r_acpi::Fadt) -> Self {
        (&f).into()
    }
}
impl From<&'_ r_acpi::Fadt> for TimerRegisterWidth {
    fn from(f: &r_acpi::Fadt) -> Self {
        let flags = f.flags;

        if flags.get_bit(8) {
            Self::Bits32
        } else {
            Self::Bits24
        }
    }
}

struct Validator(r_acpi::Fadt);
impl Validator {
    fn validate(self) -> Result<()> {
        self.validate_signature()?;
        self.validate_checksum()?;

        Ok(())
    }

    fn validate_signature(&self) -> Result<()> {
        error_unless(
            &self.0.header.signature == b"FACP",
            Error::FadtWrongSignature,
        )
    }

    fn validate_checksum(&self) -> Result<()> {
        error_unless(
            wrapping_sum_of_first_n_bytes(&self.0, self.0.header.length.try_into().unwrap()) == 0,
            Error::FadtWrongChecksum,
        )
    }
}
