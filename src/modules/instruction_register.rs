use super::*;
use crate::graphics::*;
use crate::shareable::{Shareable, Shared};
use std::fmt::{self, Display, Formatter};

#[derive(Default, Debug)]
pub struct InstructionRegister {
    value: Shareable<u8>,
}

impl InstructionRegister {
    pub fn share(&self) -> Shared<u8> {
        self.value.share()
    }
}

impl Module for InstructionRegister {
    fn get_name(&self) -> &'static str {
        "Instruction Register"
    }

    fn reset(&mut self) {
        self.value.set(0);
    }

    fn bus_write_flag(&self) -> ControlFlag {
        ControlFlag::InstructionRegisterOut
    }

    fn bus_read_flag(&self) -> ControlFlag {
        ControlFlag::InstructionRegisterIn
    }

    fn write_to_bus(&mut self) -> u8 {
        self.value.get() & 0b1111
    }

    fn read_from_bus(&mut self, bus: u8) {
        self.value.set(bus);
    }
}

impl ModuleGraphic for InstructionRegister {
    fn representation(&self) -> VisualRepresentation {
        VisualRepresentation::LedSplit(
            self.value.get(),
            LedColor::new(0.3, 0.3, 1.0),
            LedColor::new(0.9, 0.85, 0.0),
        )
    }
}

impl Display for InstructionRegister {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = self.value.get();
        write!(f, "{:04b} {:04b}", value >> 4, value & 0xf)
    }
}
