use super::*;
use crate::graphics::*;
use crate::shareable::{Share, Shareable, Shared};
use std::fmt::{self, Display, Formatter};
use std::num::Wrapping;

#[derive(Debug)]
pub struct Alu {
    pub a: Shared<u8>,
    pub b: Shared<u8>,
    pub result: u8,
    pub flags: Shareable<u8>,
}

impl Alu {
    pub fn new(a: Shared<u8>, b: Shared<u8>) -> Alu {
        Alu {
            a,
            b,
            result: 0,
            flags: Shareable::new(0b00),
        }
    }

    pub fn get_labels(&self) -> [&'static str; 8] {
        ["", "", "", "", "", "", "C", "Z"]
    }
}

impl Share<u8> for Alu {
    fn share(&self) -> Shared<u8> {
        self.flags.share()
    }
}

impl Module for Alu {
    fn get_name(&self) -> &'static str {
        "Sum Register"
    }

    fn pre_step(&mut self, cw: ControlWord) {
        let rhs = u16::from(self.b.get());
        let Wrapping(res) = Wrapping(u16::from(self.a.get())) + if cw.has(ControlFlag::Subtract) {
            Wrapping(!rhs) + Wrapping(1)
        } else {
            Wrapping(rhs)
        };
        self.result = res as u8;
        if cw.has(ControlFlag::FlagRegisterIn) {
            if res > 0xff {
                self.flags.set(self.flags.get() | 0b10);
            } else {
                self.flags.set(self.flags.get() & !0b10);
            }
            if self.result == 0 {
                self.flags.set(self.flags.get() | 0b01);
            } else {
                self.flags.set(self.flags.get() & !0b01);
            }
        }
    }

    fn reset(&mut self) {
        self.flags.set(0);
    }

    fn bus_write_flag(&self) -> ControlFlag {
        ControlFlag::SumOut
    }

    fn write_to_bus(&mut self) -> u8 {
        self.result
    }
}

impl GraphicalModule for Alu {
    fn representation(&self) -> VisualRepresentation {
        VisualRepresentation::led(self.result)
    }
}

impl Display for Alu {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:08b}", self.result)
    }
}
