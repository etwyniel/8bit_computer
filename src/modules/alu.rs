use super::{ControlFlag, ControlWord, Module};
use std::cell::RefCell;
use std::num::Wrapping;
use std::rc::Rc;
use std::default::Default;

#[derive(Debug)]
pub struct Alu {
    pub a: Rc<RefCell<u8>>,
    pub b: Rc<RefCell<u8>>,
    pub result: u8,
    pub carry: Rc<RefCell<bool>>,
    pub zero: Rc<RefCell<bool>>,
}

impl Alu {
    pub fn new(a: Rc<RefCell<u8>>, b: Rc<RefCell<u8>>) -> Alu {
        Alu { a, b, result: 0, carry: Default::default(), zero: Default::default() }
    }
}

impl Module for Alu {
    fn pre_step(&mut self, cw: ControlWord) {
        let rhs = *self.b.borrow() as u16;
        let Wrapping(res) = Wrapping(*self.a.borrow() as u16) + if cw.has(ControlFlag::Subtract) {
            Wrapping(!rhs) + Wrapping(1)
        } else {
            Wrapping(rhs)
        };
        self.result = res as u8;
        self.carry.replace(res > 0xff);
    }

    fn bus_write_flag(&self) -> ControlFlag {
        ControlFlag::SumOut
    }

    fn write_to_bus(&mut self) -> u8 {
        self.result
    }
}
