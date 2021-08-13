use {
    super::pm,
    acpi::fadt::{TimerRegisterWidth, PM_TIMER_FREQUENCY_HZ},
    apic::local::{
        lvt::{self, TimerMode},
        timer::{DivideConfiguration, DivideValue},
        CURRENT_COUNT, DIVIDE_CONFIGURATION, INITIAL_COUNT, LVT_TIMER,
    },
    kernel_mem::accessor::single::{read_only, write_only},
    x86_64::PhysAddr,
};

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
    /// This method assumes that the addresses of Local APIC registers are unchanged (that is, the
    /// start address is `0xfee0_0000`.
    pub(super) unsafe fn measure(mut self) -> u64 {
        const COUNT_MAX: u32 = u32::MAX;

        // SAFETY: The caller must not change the start address of the Local APIC registers.
        let mut lvt_timer = unsafe { write_only::<lvt::Timer>(LVT_TIMER) };
        let mut initial_count = unsafe { write_only::<u32>(INITIAL_COUNT) };
        let current_count = unsafe { read_only::<u32>(CURRENT_COUNT) };
        let mut divide_config = unsafe { write_only::<DivideConfiguration>(DIVIDE_CONFIGURATION) };

        divide_config.write_volatile(
            *DivideConfiguration::default().set_divide_value(DivideValue::DivideBy1),
        );
        lvt_timer.write_volatile(
            *lvt::Timer::default()
                .set_mask()
                .set_timer_mode(TimerMode::OneShot),
        );
        initial_count.write_volatile(COUNT_MAX);

        self.wait_milliseconds(100);

        let end_count = current_count.read_volatile();
        u64::from(COUNT_MAX - end_count) * 10 / 1000 / 1000
    }

    fn wait_milliseconds(&mut self, msec: u32) {
        if self.width == TimerRegisterWidth::Bits24 && msec >= 0x0100_0000 {
            panic!("Overflow detected.");
        }

        // Do not make `start` inline, otherwise an overflow will happen.
        let start: u64 = self.reader.read().into();
        let end = start + u64::from(PM_TIMER_FREQUENCY_HZ * msec / 1000);

        while u64::from(self.reader.read()) < end {
            core::hint::spin_loop();
        }
    }
}
