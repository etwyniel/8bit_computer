use super::*;
use crate::graphics::*;
use std::fmt::{self, Display, Formatter};
use std::num::Wrapping;

#[derive(Debug)]
pub struct ProgramCounter(pub u8);

impl Module for ProgramCounter {
    fn get_name(&self) -> &'static str {
        "Program Counter"
    }

    fn step(&mut self, cw: ControlWord, _bus: u8) {
        if cw.has(ControlFlag::CounterEnable) {
            let Wrapping(res) = Wrapping(self.0) + Wrapping(1);
            self.0 = res & 0b1111;
        }
    }

    fn reset(&mut self) {
        self.0 = 0;
    }

    fn bus_read_flag(&self) -> ControlFlag {
        ControlFlag::Jump
    }

    fn bus_write_flag(&self) -> ControlFlag {
        ControlFlag::CounterOut
    }

    fn read_from_bus(&mut self, bus: u8) {
        self.0 = bus;
    }

    fn write_to_bus(&mut self) -> u8 {
        self.0
    }
}

impl GraphicalModule for ProgramCounter {
    fn representation(&self) -> VisualRepresentation {
        VisualRepresentation::LedHalf(self.0, LedColor::new(0.0, 1.0, 0.0))
    }
}

impl Display for ProgramCounter {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:04b}", self.0)
    }
}
