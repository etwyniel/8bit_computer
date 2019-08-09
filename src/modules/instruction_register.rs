use super::{ControlFlag, Module};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default, Debug)]
pub struct InstructionRegister {
    value: Rc<RefCell<u8>>,
}

impl InstructionRegister {
    pub fn get_ref(&self) -> Rc<RefCell<u8>> {
        Rc::clone(&self.value)
    }
}

impl Module for InstructionRegister {
    fn bus_write_flag(&self) -> ControlFlag {
        ControlFlag::InstructionRegisterOut
    }

    fn bus_read_flag(&self) -> ControlFlag {
        ControlFlag::InstructionRegisterIn
    }

    fn write_to_bus(&mut self) -> u8 {
        *self.value.borrow() & 0b1111
    }

    fn read_from_bus(&mut self, bus: u8) {
        self.value.replace(bus);
    }
}
