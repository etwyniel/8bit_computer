use super::{ControlFlag, ControlWord, Module};
use crate::graphics::*;
use crate::shareable::{Shareable, Shared};
use std::fmt::{self, Display, Formatter};

/// Implementors are expected to own references to the necessary registers
pub trait InstructionDecoder {
    fn decode(&self) -> ControlWord;
    fn step(&mut self);
    fn get_counter(&self) -> usize;
    fn reset_counter(&mut self);
}

#[derive(Debug)]
pub struct SimpleInstructionDecoder {
    counter: u8,
    instruction_register: Shared<u8>,
}

#[allow(unused)]
impl SimpleInstructionDecoder {
    pub fn new(instruction_register: Shared<u8>) -> SimpleInstructionDecoder {
        SimpleInstructionDecoder {
            counter: 0,
            instruction_register,
        }
    }
}

#[allow(unused)]
impl InstructionDecoder for SimpleInstructionDecoder {
    fn decode(&self) -> ControlWord {
        use ControlFlag::*;

        let instruction = self.instruction_register.get() >> 4;
        match (instruction, self.counter) {
            (_, 0) => CounterOut | MemoryAddressIn,
            (_, 1) => RamOut | InstructionRegisterIn | CounterEnable,

            // LDA
            (0x1, 2) => InstructionRegisterOut | MemoryAddressIn,
            (0x1, 3) => RamOut | ARegisterIn,

            // ADD
            (0x2, 2) => InstructionRegisterOut | MemoryAddressIn,
            (0x2, 3) => RamOut | BRegisterIn,
            (0x2, 4) => SumOut | ARegisterIn,

            // SUB
            (0x3, 2) => InstructionRegisterOut | MemoryAddressIn,
            (0x3, 3) => RamOut | BRegisterIn,
            (0x3, 4) => Subtract | SumOut | ARegisterIn,

            // STA
            (0x4, 2) => InstructionRegisterOut | MemoryAddressIn,
            (0x4, 3) => ARegisterOut | RamIn,

            // LDI
            (0x5, 2) => InstructionRegisterOut | ARegisterIn,

            //JMP
            (0x6, 2) => InstructionRegisterOut | Jump,

            // OUT
            (0xe, 2) => ARegisterOut | OutputRegisterIn,

            // Hlt
            (0xf, 2) => ControlWord(Hlt as u32),
            _ => ControlWord(0),
        }
    }

    fn step(&mut self) {
        self.counter = (self.counter + 1) % 5;
    }

    fn get_counter(&self) -> usize {
        self.counter as usize
    }

    fn reset_counter(&mut self) {
        self.counter = 0;
    }
}

pub struct BranchingInstructionDecoder {
    counter: Shareable<u8>,
    instruction_register: Shared<u8>,
    carry: Shared<bool>,
    zero: Shared<bool>,
}

impl BranchingInstructionDecoder {
    pub fn new(instruction_register: Shared<u8>, carry: Shared<bool>, zero: Shared<bool>) -> Self {
        BranchingInstructionDecoder {
            counter: Shareable::new(0),
            instruction_register,
            carry,
            zero,
        }
    }

    pub fn share_counter(&self) -> Shared<u8> {
        self.counter.share()
    }
}

impl InstructionDecoder for BranchingInstructionDecoder {
    fn decode(&self) -> ControlWord {
        use ControlFlag::*;

        let instruction = self.instruction_register.get() >> 4;
        let carry = self.carry.get();
        let zero = self.zero.get();
        match (instruction, self.counter.get()) {
            (_, 0) => CounterOut | MemoryAddressIn,
            (_, 1) => RamOut | InstructionRegisterIn | CounterEnable,

            // LDA
            (0x1, 2) => InstructionRegisterOut | MemoryAddressIn,
            (0x1, 3) => RamOut | ARegisterIn | NextInstruction,

            // ADD
            (0x2, 2) => InstructionRegisterOut | MemoryAddressIn,
            (0x2, 3) => RamOut | BRegisterIn,
            (0x2, 4) => SumOut | ARegisterIn | NextInstruction,

            // SUB
            (0x3, 2) => InstructionRegisterOut | MemoryAddressIn,
            (0x3, 3) => RamOut | BRegisterIn,
            (0x3, 4) => Subtract | SumOut | ARegisterIn | NextInstruction,

            // STA
            (0x4, 2) => InstructionRegisterOut | MemoryAddressIn,
            (0x4, 3) => ARegisterOut | RamIn | NextInstruction,

            // LDI
            (0x5, 2) => InstructionRegisterOut | ARegisterIn | NextInstruction,

            //JMP
            (0x6, 2) => InstructionRegisterOut | Jump | NextInstruction,

            // JC
            (0x7, 2) if carry => InstructionRegisterOut | Jump | NextInstruction,
            (0x7, 2) => Empty | NextInstruction,

            // JZ
            (0x8, 2) if zero => InstructionRegisterOut | Jump | NextInstruction,
            (0x8, 2) => Empty | NextInstruction,

            // OUT
            (0xe, 2) => ARegisterOut | OutputRegisterIn | NextInstruction,

            // Hlt
            (0xf, 2) => ControlWord(Hlt as u32),
            _ => ControlWord(0),
        }
    }

    fn step(&mut self) {
        self.counter.set((self.counter.get() + 1) % 5);
    }

    fn get_counter(&self) -> usize {
        self.counter.get() as usize
    }

    fn reset_counter(&mut self) {
        self.counter.set(0);
    }
}

#[derive(Debug)]
pub struct DecoderStep(pub Shared<u8>);

impl Module for DecoderStep {
    fn get_name(&self) -> &'static str {
        "Decoder Step"
    }

    fn reset(&mut self) {}
}

impl Display for DecoderStep {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:03b}", self.0.get())
    }
}

impl ModuleGraphic for DecoderStep {
    fn representation(&self) -> VisualRepresentation {
        VisualRepresentation::LedN(self.0.get() as usize, 3, LedColor::default())
    }
}
