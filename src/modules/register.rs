use super::{ControlFlag, Module};
use crate::shareable::{Shareable, Shared};

#[derive(Debug)]
pub struct Register {
    value: Shareable<u8>,
    in_flag: ControlFlag,
    out_flag: ControlFlag,
}

impl Register {
    pub fn new(in_flag: ControlFlag, out_flag: ControlFlag) -> Register {
        Register {
            value: Shareable::new(0),
            in_flag,
            out_flag,
        }
    }

    pub fn new_ro(in_flag: ControlFlag) -> Register {
        Self::new(in_flag, ControlFlag::Empty)
    }

    pub fn share(&self) -> Shared<u8> {
        self.value.share()
    }

    pub fn set(&mut self, value: u8) {
        self.value.set(value);
    }
}

impl Module for Register {
    fn reset(&mut self) {
        self.value.set(0);
    }

    fn bus_read_flag(&self) -> ControlFlag {
        self.in_flag
    }

    fn bus_write_flag(&self) -> ControlFlag {
        self.out_flag
    }

    fn read_from_bus(&mut self, bus: u8) {
        self.value.set(bus);
    }

    fn write_to_bus(&mut self) -> u8 {
        self.value.get()
    }
}
