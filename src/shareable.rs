use std::cell::Cell;
use std::rc::Rc;

pub trait Share<T: Copy> {
    fn share(&self) -> Shared<T>;
}

#[derive(Debug, Default)]
pub struct Shareable<T: Copy>(Rc<Cell<T>>);

#[derive(Debug)]
pub struct Shared<T: Copy>(Rc<Cell<T>>);

impl<T: Copy> Shareable<T> {
    pub fn new(init: T) -> Self {
        Shareable(Rc::new(Cell::new(init)))
    }

    pub fn set(&mut self, value: T) {
        self.0.replace(value);
    }

    pub fn get(&self) -> T {
        self.0.get()
    }
}

impl<T: Copy> Share<T> for Shareable<T> {
    fn share(&self) -> Shared<T> {
        Shared(Rc::clone(&self.0))
    }
}

impl<T: Copy> Shared<T> {
    pub fn get(&self) -> T {
        self.0.get()
    }
}

impl<T: Copy> Share<T> for Shared<T> {
    fn share(&self) -> Self {
        Shared(Rc::clone(&self.0))
    }
}
