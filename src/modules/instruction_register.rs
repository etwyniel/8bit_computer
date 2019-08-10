use super::{ControlFlag, Module};
use crate::shareable::{Shareable, Shared};

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
