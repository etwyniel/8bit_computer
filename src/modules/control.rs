use std::ops::BitOr;
use std::fmt::{self, Display, Formatter};
use crate::modules::control::ControlFlag::NextInstruction;

#[repr(u32)]
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
    NextInstruction = 1 << 16,
}

impl BitOr for ControlFlag {
    type Output = ControlWord;

    fn bitor(self, rhs: ControlFlag) -> ControlWord {
        ControlWord(self as u32 | rhs as u32)
    }
}

#[derive(Clone, Copy, Default)]
pub struct ControlWord(pub u32);

impl ControlWord {
    pub fn has(self, flag: ControlFlag) -> bool {
        self.0 & (flag as u32) > 0
    }
}

impl Display for ControlWord {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use ControlFlag::*;

        const FLAGS: [ControlFlag; 18] = [
            Empty,
            Hlt,
            MemoryAddressIn,
            RamIn,
            RamOut,
            InstructionRegisterOut,
            InstructionRegisterIn,
            ARegisterIn,
            ARegisterOut,
            SumOut,
            Subtract,
            BRegisterIn,
            OutputRegisterIn,
            CounterEnable,
            CounterOut,
            Jump,
            FlagRegisterIn,
            NextInstruction,
        ];

        let mut flags = Vec::new();
        for flag in &FLAGS {
            if self.has(*flag) {
                flags.push(*flag);
            }
        }
        if flags.is_empty() {
            write!(f, "Empty")
        } else if flags.len() == 1 {
            write!(f, "{:?}", flags[0])
        } else {
            for flag in flags.iter().take(flags.len() - 1) {
                write!(f, "{:?} | ", flag)?;
            }
            write!(f, "{:?}", flags[flags.len() - 1])
        }
    }
}

impl BitOr<ControlFlag> for ControlWord {
    type Output = Self;

    fn bitor(self, flag: ControlFlag) -> Self {
        ControlWord(self.0 | flag as u32)
    }
}
