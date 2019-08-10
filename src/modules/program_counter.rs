use super::{ControlFlag, ControlWord, Module};
use std::num::Wrapping;

#[derive(Debug)]
pub struct ProgramCounter(pub u8);

impl Module for ProgramCounter {
    fn step(&mut self, cw: ControlWord, _bus: u8) {
        if cw.has(ControlFlag::CounterEnable) {
            let Wrapping(res) = Wrapping(self.0) + Wrapping(1);
            self.0 = res & 0b1111;
        }
    }

    fn reset(&mut self) {
        self.0 = 0;
    }

    fn bus_write_flag(&self) -> ControlFlag {
        ControlFlag::CounterOut
    }

    fn bus_read_flag(&self) -> ControlFlag {
        ControlFlag::Jump
    }

    fn write_to_bus(&mut self) -> u8 {
        self.0
    }

    fn read_from_bus(&mut self, bus: u8) {
        self.0 = bus;
    }
}
