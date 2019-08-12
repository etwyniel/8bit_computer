use super::*;
use crate::graphics::*;
use std::fmt::{self, Display, Formatter};

#[derive(Default, Debug)]
pub struct OutputRegister(pub u8);

impl Module for OutputRegister {
    fn get_name(&self) -> &'static str {
        "Output"
    }

    fn reset(&mut self) {
        self.0 = 0;
    }

    fn bus_read_flag(&self) -> ControlFlag {
        ControlFlag::OutputRegisterIn
    }

    fn read_from_bus(&mut self, bus: u8) {
        self.0 = bus;
    }
}

impl ModuleGraphic for OutputRegister {
    fn representation(&self) -> VisualRepresentation {
        VisualRepresentation::Text(format!("{}", self.0))
    }
}

impl Display for OutputRegister {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
