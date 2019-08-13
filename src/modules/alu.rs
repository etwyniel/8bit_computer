use super::*;
use crate::graphics::*;
use crate::shareable::{Shareable, Shared};
use std::default::Default;
use std::fmt::{self, Display, Formatter};
use std::num::Wrapping;

#[derive(Debug)]
pub struct Alu {
    pub a: Shared<u8>,
    pub b: Shared<u8>,
    pub result: u8,
    pub carry: Shareable<bool>,
    pub zero: Shareable<bool>,
}

impl Alu {
    pub fn new(a: Shared<u8>, b: Shared<u8>) -> Alu {
        Alu {
            a,
            b,
            result: 0,
            carry: Default::default(),
            zero: Default::default(),
        }
    }

    pub fn share_carry(&self) -> Shared<bool> {
        self.carry.share()
    }

    pub fn share_zero(&self) -> Shared<bool> {
        self.zero.share()
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
        self.carry.set(res > 0xff);
    }

    fn reset(&mut self) {
        self.carry.set(false);
        self.zero.set(false);
    }

    fn bus_write_flag(&self) -> ControlFlag {
        ControlFlag::SumOut
    }

    fn write_to_bus(&mut self) -> u8 {
        self.result
    }
}

impl ModuleGraphic for Alu {
    fn representation(&self) -> VisualRepresentation {
        VisualRepresentation::led(self.result)
    }
}

impl Display for Alu {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:08b}", self.result)
    }
}
