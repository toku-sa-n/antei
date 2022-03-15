use {
    super::pm,
    acpi::fadt::{TimerRegisterWidth, PM_TIMER_FREQUENCY_HZ},
    apic::local::{
        lvt::{self, TimerMode},
        timer::{DivideConfiguration, DivideValue},
        CURRENT_COUNT, DIVIDE_CONFIGURATION, INITIAL_COUNT, LVT_TIMER,
    },
    core::convert::TryInto,
    vm::accessor::single::{read_only, write_only},
    x86_64::PhysAddr,
};

/// # Safety
///
/// - `rsdp` must be the correct address of RSDP.
/// - The start address of the Local APIC registers must be `0xfee0_0000` (the default one).
pub(super) unsafe fn init(rsdp: PhysAddr) {
    // SAFETY: The caller must ensure that `rsdp` is the correct address of RSDP and the start
    // address of the Local APIC registers must be `0xfee0_0000`.
    let frequency = unsafe { get_frequency(rsdp) };

    // SAFETY: The caller must ensure that the start address of the Local APIC registers must be
    // `0xfee0_0000`.
    unsafe {
        enable_interrupts(0x20, (frequency / 100).try_into().unwrap());
    }
}

/// # Safety
///
/// - `rsdp` must be the correct address of RSDP.
/// - The start address of the Local APIC registers must be `0xfee0_0000` (the default one).
unsafe fn get_frequency(rsdp: PhysAddr) -> u64 {
    // SAFETY: The caller must ensure that `rsdp` is the correct address of RSDP.
    let measurer = unsafe { FrequencyMeasurer::from_rsdp_addr(rsdp) };

    // SAFETY: The caller must not change the start adress of the Local APIC registers.
    unsafe { measurer.measure_hz() }
}

/// # Safety
///
/// The caller must ensure that the start address of the Local APIC registers must be `0xfee0_0000`
/// (the default one).
unsafe fn enable_interrupts(vector: u8, initial_count: u32) {
    // SAFETY: The caller must ensure that the start address of the Local APIC registers must be
    // `0xfee0_0000`. (the default one)
    unsafe {
        set_lvt_timer(
            *lvt::Timer::default()
                .set_vector(vector)
                .set_timer_mode(TimerMode::Periodic),
        );
        set_divide_config(*DivideConfiguration::default().set_divide_value(DivideValue::DivideBy1));
        set_initial_count(initial_count);
    }
}

pub(super) struct FrequencyMeasurer {
    reader: pm::RegisterReader,
    width: TimerRegisterWidth,
}
impl FrequencyMeasurer {
    /// # Safety
    ///
    /// `rsdp` must be the correct address of RSDP.
    pub(super) unsafe fn from_rsdp_addr(rsdp: PhysAddr) -> Self {
        // SAFETY: The caller must ensure that `rsdp` is the correct address of RSDP.
        let timer = unsafe { pm::timer_info_from_rsdp_addr(rsdp) };

        Self {
            reader: pm::RegisterReader::new(&timer),
            width: timer.width(),
        }
    }

    /// # Safety
    ///
    /// The address of the Local APIC registers must be `0xfee0_0000` (the default one).
    pub(super) unsafe fn measure_hz(mut self) -> u64 {
        const COUNT_MAX: u32 = u32::MAX;
        const TIME_TO_ELAPSE: u32 = 100;
        const SEC_IN_MSEC: u32 = 1000;

        // SAFETY: The caller must ensure that the start address of the Local APIC registers is
        // `0xfee0_0000`.
        unsafe {
            set_divide_config(
                *DivideConfiguration::default().set_divide_value(DivideValue::DivideBy1),
            );
            set_lvt_timer(
                *lvt::Timer::default()
                    .set_mask()
                    .set_timer_mode(TimerMode::OneShot),
            );
            set_initial_count(COUNT_MAX);
        }

        self.wait_milliseconds(TIME_TO_ELAPSE);

        // SAFETY: The caller must ensure that the start address of the Local APIC register is
        // `0xfee0_0000`.
        let end_count = unsafe { current_count() };

        u64::from(COUNT_MAX - end_count) * u64::from(SEC_IN_MSEC / TIME_TO_ELAPSE)
    }

    fn wait_milliseconds(&mut self, msec: u32) {
        assert!(
            !(self.width == TimerRegisterWidth::Bits24 && msec >= 0x0100_0000),
            "Overflow detected."
        );

        // Do not make `start` inline, otherwise an overflow will happen.
        let start: u64 = self.reader.read().into();
        let end = start + u64::from(PM_TIMER_FREQUENCY_HZ * msec / 1000);

        while u64::from(self.reader.read()) < end {
            core::hint::spin_loop();
        }
    }
}

/// # Safety
///
/// The start address of the Local APIC registers must be `0xfee0_0000`. (the default one)
unsafe fn current_count() -> u32 {
    unsafe { read_only(CURRENT_COUNT).read_volatile() }
}

/// # Safety
///
/// The start address of the Local APIC registers must be `0xfee0_0000`. (the default one)
unsafe fn set_lvt_timer(lvt_timer: lvt::Timer) {
    // SAFETY: The caller must ensure that the start address of the Local APIC registers must be
    // `0xfee0_0000`.
    unsafe {
        write_only(LVT_TIMER).write_volatile(lvt_timer);
    }
}

/// # Safety
///
/// The start address of the Local APIC registers must be `0xfee0_0000`. (the default one)
unsafe fn set_divide_config(config: DivideConfiguration) {
    // SAFETY: The caller must ensure that the start address of the Local APIC registers must be
    // `0xfee0_0000`.
    unsafe {
        write_only(DIVIDE_CONFIGURATION).write_volatile(config);
    }
}

/// # Safety
///
/// The start address of the Local APIC registers must be `0xfee0_0000`. (the default one)
unsafe fn set_initial_count(count: u32) {
    // SAFETY: The caller must ensure that the start address of the Local APIC registers must be
    // `0xfee0_0000`.
    unsafe {
        write_only(INITIAL_COUNT).write_volatile(count);
    }
}
