use super::{ControlFlag, ControlWord, Module};
use std::cell::RefCell;
use std::default::Default;
use std::rc::Rc;

#[derive(Default, Debug)]
pub struct Ram {
    address: Rc<RefCell<u8>>,
    pub memory: [u8; 16],
    byte: Rc<RefCell<u8>>,
}

impl Ram {
    pub fn new(address: Rc<RefCell<u8>>) -> Ram {
        Ram {
            address,
            ..Self::default()
        }
    }
}

impl Module for Ram {
    fn pre_step(&mut self, _cw: ControlWord) {
        self.byte
            .replace(self.memory[*self.address.borrow() as usize]);
    }

    fn bus_read_flag(&self) -> ControlFlag {
        ControlFlag::RamIn
    }

    fn bus_write_flag(&self) -> ControlFlag {
        ControlFlag::RamOut
    }

    fn read_from_bus(&mut self, bus: u8) {
        self.memory[*self.address.borrow() as usize] = bus;
    }

    fn write_to_bus(&mut self) -> u8 {
        self.memory[*self.address.borrow() as usize]
    }
}
