use atty::Stream;
use crate::graphics::GraphicalModule;
use crate::modules::*;
use crate::shareable::{Share, Shared};
use std::convert::AsRef;
use std::default::Default;
use std::path::Path;

pub type Modules = Vec<Box<dyn GraphicalModule>>;

pub struct BreadboardState<I: InstructionDecoder = BranchingInstructionDecoder> {
    modules: Modules,
    decoder: I,
    bus: u8,
    cw: ControlWord,
    output: Vec<(String, String)>,
}

impl Default for BreadboardState {
    fn default() -> Self {
        Self::default_with_ram(write_sample_program)
    }
}

impl BreadboardState {
    pub fn default_with_ram<F: FnOnce(&mut [u8; 16])>(
        f: F,
    ) -> BreadboardState<BranchingInstructionDecoder> {
        Self::default_with_decoder(f, |i, f| Ok(BranchingInstructionDecoder::new(i, f))).unwrap()
    }

    pub fn from_microcode<P, F>(
        microcode_path: P,
        ram_init: F,
    ) -> Result<BreadboardState<MicrocodeDecoder>, String>
    where
        P: AsRef<Path>,
        F: FnOnce(&mut [u8; 16]),
    {
        Self::default_with_decoder(
            ram_init,
            |instruction, flags| {
                MicrocodeDecoder::from_file(instruction, flags, microcode_path).map_err(|e| e.to_string())
            },
        )
    }

    pub fn default_with_decoder<R, D, I>(ram_init: R, get_decoder: D) -> Result<BreadboardState<I>, String>
    where
        R: FnOnce(&mut [u8; 16]),
        D: FnOnce(Shared<u8>, Shared<u8>) -> Result<I, String>,
        I: InstructionDecoder + Share<u8>,
    {
        let a = Register::new(
            "A Register",
            ControlFlag::ARegisterIn,
            ControlFlag::ARegisterOut,
        );
        let b = Register::new_ro("B Register", ControlFlag::BRegisterIn);
        let alu = Alu::new(a.share(), b.share());
        let flags_register = FlagsRegister::new(alu.share(), alu.get_labels());
        let output = OutputRegister(0);
        let address_register = Register::new_ro("Memory Address", ControlFlag::MemoryAddressIn);
        let mut ram = Ram::new(address_register.share());
        ram_init(&mut ram.memory);
        let instruction_register = InstructionRegister::default();
        let instruction = instruction_register.share();
        let program_counter = ProgramCounter(0);
        let decoder = get_decoder(instruction, alu.share())?;
        let decoder_step = DecoderStep(decoder.share());
        let modules: Modules = vec![
            Box::new(program_counter),
            Box::new(address_register),
            Box::new(ram),
            Box::new(instruction_register),
            Box::new(decoder_step),
            Box::new(a),
            Box::new(alu),
            Box::new(flags_register),
            Box::new(b),
            Box::new(output),
        ];
        Ok(BreadboardState::new(modules, decoder))
    }
}

impl<I: InstructionDecoder> BreadboardState<I> {
    pub fn new(modules: Modules, decoder: I) -> Self {
        let output = Vec::with_capacity(modules.len());
        BreadboardState {
            modules,
            decoder,
            bus: 0,
            cw: ControlWord(0),
            output,
        }
    }

    pub fn modules(&self) -> &Modules {
        &self.modules
    }

    pub fn bus(&self) -> u8 {
        self.bus
    }

    pub fn cw(&self) -> ControlWord {
        self.cw
    }

    pub fn reset(&mut self) {
        for module in self.modules.iter_mut() {
            module.reset();
        }
        self.decoder.reset_counter();
    }

    pub fn falling_edge(&mut self) {
        self.decoder.step();
        if self.cw.has(ControlFlag::NextInstruction) {
            self.decoder.reset_counter();
        }
    }

    pub fn rising_edge(&mut self) {
        if self.cw.has(ControlFlag::Hlt) {
            return;
        }
        for module in self.modules.iter_mut() {
            module.step(self.cw, self.bus);
        }

        for module in self.modules.iter_mut() {
            module.bus_read(self.cw, self.bus);
        }
    }

    pub fn pre_step(&mut self) {
        self.cw = self.decoder.decode();
        for module in self.modules.iter_mut() {
            module.pre_step(self.cw);
        }
        let mut maybe_bus = None;
        let cw = self.cw;
        for module in self.modules.iter_mut() {
            maybe_bus = maybe_bus.or_else(|| module.bus_write(cw));
        }
        self.bus = maybe_bus.unwrap_or(0);
        self.output.clear();

        for module in self.modules.iter() {
            self.output
                .push((module.get_name().to_string(), format!("{}", module)));
        }
        self.output
            .push(("Control word".to_string(), format!("{}", self.cw)));
        self.output
            .push(("Bus".to_string(), format!("{:08b}", self.bus)));
        self.pretty_print_output();
    }

    pub fn pretty_print_output(&self) {
        let longest = self
            .output
            .iter()
            .map(|(name, _)| name.len())
            .max()
            .unwrap_or(0);
        for (ref name, ref contents) in self.output.iter() {
            if name.is_empty() && contents.is_empty() {
                continue;
            }
            if atty::is(Stream::Stdout) {
                println!(
                    "\x1b[1;32m{:>width$}\x1b[0m {}",
                    name,
                    contents,
                    width = longest
                );
            } else {
                println!("{:>width$} {}", name, contents, width = longest);
            }
        }
        println!();
    }
}

pub fn write_sample_program(ram: &mut [u8; 16]) {
    // Increments A to 255 then decrements it down to 0 and repeats
    ram[0x0] = 0xe0; // OUT
    ram[0x1] = 0x2f; // ADD 15
    ram[0x2] = 0x74; // JC 4
    ram[0x3] = 0x60; // JMP 0
    ram[0x4] = 0x3f; // SUB 15
    ram[0x5] = 0xe0; // OUT
    ram[0x6] = 0x80; // JZ 0
    ram[0x7] = 0x64; // JMP 4
    ram[0xf] = 1;
}
