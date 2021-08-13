use {bit_field::BitField, num_derive::FromPrimitive, num_traits::FromPrimitive};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct DivideConfiguration(u32);
impl DivideConfiguration {
    #[must_use]
    pub fn divide_value(self) -> DivideValue {
        FromPrimitive::from_u32(self.0.get_bits(0..4)).expect("Invalid divide value.")
    }

    pub fn set_divide_value(&mut self, value: DivideValue) -> &mut Self {
        self.0.set_bits(0..4, value as _);
        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum DivideValue {
    DivideBy2 = 0b0000,
    DivideBy4 = 0b0001,
    DivideBy8 = 0b0010,
    DivideBy16 = 0b0011,
    DivideBy32 = 0b1000,
    DivideBy64 = 0b1001,
    DivideBy128 = 0b1010,
    DivideBy1 = 0b1011,
}
