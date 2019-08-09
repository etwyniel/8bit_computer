use std::ops::BitOr;

#[repr(u16)]
#[derive(Copy, Clone, Debug)]
pub enum ControlFlag {
    Empty = 0,
    Hlt = 1 << 0,
    MemoryAddressIn = 1 << 1,
    RamIn = 1 << 2,
    RamOut = 1 << 3,
    InstructionRegisterOut = 1 << 4,
    InstructionRegisterIn = 1 << 5,
    ARegisterIn = 1 << 6,
    ARegisterOut = 1 << 7,
    SumOut = 1 << 8,
    Subtract = 1 << 9,
    BRegisterIn = 1 << 10,
    OutputRegisterIn = 1 << 11,
    CounterEnable = 1 << 12,
    CounterOut = 1 << 13,
    Jump = 1 << 14,
    FlagRegisterIn = 1 << 15,
}

impl BitOr for ControlFlag {
    type Output = ControlWord;

    fn bitor(self, rhs: ControlFlag) -> ControlWord {
        ControlWord(self as u16 | rhs as u16)
    }
}

#[derive(Clone, Copy, Default)]
pub struct ControlWord(pub u16);

impl ControlWord {
    pub fn has(self, flag: ControlFlag) -> bool {
        self.0 & (flag as u16) > 0
    }
}

impl BitOr<ControlFlag> for ControlWord {
    type Output = Self;

    fn bitor(self, flag: ControlFlag) -> Self {
        ControlWord(self.0 | flag as u16)
    }
}
