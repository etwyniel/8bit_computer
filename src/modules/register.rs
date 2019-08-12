use super::*;
use crate::graphics::*;
use crate::shareable::{Shareable, Shared};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Register {
    value: Shareable<u8>,
    name: String,
    in_flag: ControlFlag,
    out_flag: ControlFlag,
}

impl Register {
    pub fn new(name: &str, in_flag: ControlFlag, out_flag: ControlFlag) -> Register {
        Register {
            value: Shareable::new(0),
            name: name.to_string(),
            in_flag,
            out_flag,
        }
    }

    pub fn new_ro(name: &str, in_flag: ControlFlag) -> Register {
        Self::new(name, in_flag, ControlFlag::Empty)
    }

    pub fn share(&self) -> Shared<u8> {
        self.value.share()
    }
}

impl Module for Register {
    fn get_name(&self) -> &str {
        &self.name
    }

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

impl ModuleGraphic for Register {
    fn representation(&self) -> VisualRepresentation {
        VisualRepresentation::led(self.value.get())
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:08b}", self.value.get())
    }
}
