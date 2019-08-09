use super::{ControlFlag, Module};

#[derive(Default, Debug)]
pub struct OutputRegister(pub u8);

impl Module for OutputRegister {
    fn bus_read_flag(&self) -> ControlFlag {
        ControlFlag::OutputRegisterIn
    }

    fn read_from_bus(&mut self, bus: u8) {
        self.0 = bus;
        println!("Output: {}", self.0);
    }
}
