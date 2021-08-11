use {
    bit_field::BitField, core::convert::TryInto, num_derive::FromPrimitive,
    num_traits::FromPrimitive, paste::paste,
};

macro_rules! bit {
    ($name:ident,$pos:expr) => {
        paste! {
            #[must_use]
            pub fn $name(&self) -> bool {
                self.0.get_bit($pos)
            }

            pub fn [<set_ $name>](&mut self) -> &mut Self {
                self.0.set_bit($pos, true);
                self
            }

            pub fn [<clear_ $name>](&mut self) -> &mut Self {
                self.0.set_bit($pos, false);
                self
            }
        }
    };
}

macro_rules! vector {
    () => {
        #[must_use]
        pub fn vector(&self) -> u8 {
            self.0.get_bits(0..8).try_into().unwrap()
        }

        pub fn set_vector(&mut self, vector: u8) -> &mut Self {
            self.0.set_bits(0..8, vector.into());
            self
        }
    };
}

macro_rules! delivery_mode {
    () => {
        #[must_use]
        pub fn delivery_mode(&self) -> DeliveryMode {
            FromPrimitive::from_u32(self.0.get_bits(8..11)).expect("Invalid delivery mode")
        }

        pub fn set_delivery_mode(&mut self, delivery_mode: DeliveryMode) -> &mut Self {
            self.0.set_bits(8..11, delivery_mode as _);
            self
        }
    };
}

macro_rules! delivery_status {
    () => {
        bit!(delivery_status, 12);
    };
}

macro_rules! interrupt_input_pin_polarity {
    () => {
        bit!(interrupt_input_pin_polarity, 13);
    };
}

macro_rules! remote_irr {
    () => {
        bit!(remote_irr, 14);
    };
}

macro_rules! trigger_mode {
    () => {
        bit!(trigger_mode, 15);
    };
}

macro_rules! mask {
    () => {
        bit!(mask, 16);
    };
}

macro_rules! timer_mode {
    () => {
        #[must_use]
        pub fn timer_mode(&self) -> TimerMode {
            FromPrimitive::from_u32(self.0.get_bits(17..19)).expect("Invalid timer mode")
        }

        pub fn set_timer_mode(&mut self, timer_mode: TimerMode) -> &mut Self {
            self.0.set_bits(17..19, timer_mode as _);
            self
        }
    };
}

macro_rules! lvt {
    ($name:ident{
        $($field:ident),*$(,)?
    }) => {
        #[derive(Copy,Clone,Debug,Default,PartialEq,Eq)]
        pub struct $name(u32);
        impl $name{
            $($field!();)*
        }
    };
}

lvt! {
    Timer {
        vector,
        delivery_status,
        mask,
        timer_mode,
    }
}

lvt! {
    Cmci {
        vector,
        delivery_mode,
        delivery_status,
        mask,
    }
}

lvt! {
    Lint0 {
        vector,
        delivery_mode,
        delivery_status,
        interrupt_input_pin_polarity,
        remote_irr,
        trigger_mode,
        mask,
    }
}

lvt! {
    Lint1 {
        vector,
        delivery_mode,
        delivery_status,
        interrupt_input_pin_polarity,
        remote_irr,
        trigger_mode,
        mask,
    }
}

lvt! {
    Error {
        vector,
        delivery_status,
        mask,
    }
}

lvt! {
    PerformanceMonitoringCounters {
        vector,
        delivery_mode,
        delivery_status,
        mask,
    }
}

lvt! {
    ThermalSensor {
        vector,
        delivery_mode,
        delivery_status,
        mask,
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum DeliveryMode {
    Fixed = 0b000,
    Smi = 0b010,
    Nmi = 0b100,
    ExtInt = 0b111,
    Int = 0b101,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum TimerMode {
    OneShot = 0b00,
    Periodic = 0b01,
    TscDeadline = 0b10,
}
