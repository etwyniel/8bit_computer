use super::{ControlFlag, Module};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Register {
    value: Rc<RefCell<u8>>,
    in_flag: ControlFlag,
    out_flag: ControlFlag,
}

impl Register {
    pub fn new(in_flag: ControlFlag, out_flag: ControlFlag) -> Register {
        Register {
            value: Rc::new(RefCell::new(0)),
            in_flag,
            out_flag,
        }
    }

    pub fn new_ro(in_flag: ControlFlag) -> Register {
        Self::new(in_flag, ControlFlag::Empty)
    }

    pub fn get_ref(&self) -> Rc<RefCell<u8>> {
        Rc::clone(&self.value)
    }

    pub fn set(&mut self, value: u8) {
        self.value.replace(value);
    }
}

impl Module for Register {
    fn bus_read_flag(&self) -> ControlFlag {
        self.in_flag
    }

    fn bus_write_flag(&self) -> ControlFlag {
        self.out_flag
    }

    fn read_from_bus(&mut self, bus: u8) {
        self.value.replace(bus);
    }

    fn write_to_bus(&mut self) -> u8 {
        *self.value.borrow()
    }
}
