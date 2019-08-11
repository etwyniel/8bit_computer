use std::cell::Cell;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct Shareable<T: Copy>(Rc<Cell<T>>);

#[derive(Debug)]
pub struct Shared<T: Copy>(Rc<Cell<T>>);

impl<T: Copy> Shareable<T> {
    pub fn new(init: T) -> Self {
        Shareable(Rc::new(Cell::new(init)))
    }

    pub fn share(&self) -> Shared<T> {
        Shared(Rc::clone(&self.0))
    }

    pub fn set(&mut self, value: T) {
        self.0.replace(value);
    }

    pub fn get(&self) -> T {
        self.0.get()
    }
}

impl<T: Copy> Shared<T> {
    pub fn get(&self) -> T {
        self.0.get()
    }
}
