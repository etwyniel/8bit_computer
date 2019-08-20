use super::*;
use crate::graphics::*;
use crate::shareable::Shared;
use std::fmt::{self, Display, Formatter};

// Latches in the flags from the ALU
#[derive(Debug)]
pub struct FlagsRegister {
    flags: Shared<u8>,
    labels: [&'static str; 8],
}

impl FlagsRegister {
    pub fn new(flags: Shared<u8>, labels: [&'static str; 8]) -> Self {
        FlagsRegister { flags, labels }
    }
}

impl Display for FlagsRegister {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut active: Vec<&str> = Vec::new();
        let flags = self.flags.get();
        for bit in 0..8 {
            if flags & (1 << (7 - bit)) > 0 {
                active.push(self.labels[bit]);
            }
        }
        write!(f, "{}", &active.join(" | "))
    }
}

impl Module for FlagsRegister {
    fn get_name(&self) -> &str {
        "Flags"
    }

    fn reset(&mut self) {}
}

impl GraphicalModule for FlagsRegister {
    fn representation(&self) -> VisualRepresentation {
        let num_flags = self
            .labels
            .iter()
            .rev()
            .take_while(|s| !s.is_empty())
            .count();
        VisualRepresentation::LabeledLedN(
            self.flags.get() as usize,
            num_flags as u8,
            LedColor::new(0.0, 1.0, 0.0),
            &self.labels[8 - num_flags..],
        )
    }
}
