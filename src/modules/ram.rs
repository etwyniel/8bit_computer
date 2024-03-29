use super::*;
use crate::graphics::*;
use crate::shareable::Shared;
use std::default::Default;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Ram {
    address: Shared<u8>,
    pub memory: [u8; 16],
    byte: u8,
}

impl Ram {
    pub fn new(address: Shared<u8>) -> Ram {
        Ram {
            address,
            memory: [0; 16],
            byte: Default::default(),
        }
    }
}

impl Module for Ram {
    fn get_name(&self) -> &'static str {
        "Memory Contents"
    }

    fn pre_step(&mut self, _cw: ControlWord) {
        self.byte = self.memory[self.address.get() as usize];
    }

    fn reset(&mut self) {}

    fn bus_read_flag(&self) -> ControlFlag {
        ControlFlag::RamIn
    }

    fn bus_write_flag(&self) -> ControlFlag {
        ControlFlag::RamOut
    }

    fn read_from_bus(&mut self, bus: u8) {
        self.memory[self.address.get() as usize] = bus;
    }

    fn write_to_bus(&mut self) -> u8 {
        self.byte
    }
}

impl GraphicalModule for Ram {
    fn representation(&self) -> VisualRepresentation {
        VisualRepresentation::led(self.byte)
    }
}

impl Display for Ram {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:08b}", self.byte)
    }
}
