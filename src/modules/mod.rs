pub mod alu;
pub mod control;
pub mod instruction_decoder;
pub mod instruction_register;
pub mod output_register;
pub mod program_counter;
pub mod ram;
pub mod register;

pub use alu::Alu;
pub use control::{ControlFlag, ControlWord};
pub use instruction_decoder::{InstructionDecoder, SimpleInstructionDecoder, BranchingInstructionDecoder};
pub use instruction_register::InstructionRegister;
pub use output_register::OutputRegister;
pub use program_counter::ProgramCounter;
pub use ram::Ram;
pub use register::Register;

pub trait Module: std::fmt::Debug {
    fn pre_step(&mut self, _cw: ControlWord) {}
    fn step(&mut self, _cw: ControlWord, _bus: u8) {}
    fn reset(&mut self);

    fn bus_read_flag(&self) -> ControlFlag {
        ControlFlag::Empty
    }
    fn bus_write_flag(&self) -> ControlFlag {
        ControlFlag::Empty
    }
    fn read_from_bus(&mut self, _bus: u8) {}

    /// Not guaranteed to run. Since it is invalid to put mutliple values on
    /// the bus at once, this function can be short-circuited
    fn write_to_bus(&mut self) -> u8 {
        0
    }

    fn bus_write(&mut self, cw: ControlWord) -> Option<u8> {
        if cw.has(self.bus_write_flag()) {
            Some(self.write_to_bus())
        } else {
            None
        }
    }

    fn bus_read(&mut self, cw: ControlWord, bus: u8) {
        if cw.has(self.bus_read_flag()) {
            self.read_from_bus(bus);
        }
    }
}
