use crate::graphics::GraphicalModule;
use crate::modules::*;
use std::default::Default;
use atty::Stream;

type Modules = Vec<Box<dyn GraphicalModule>>;

pub struct BreadboardState<I: InstructionDecoder = BranchingInstructionDecoder> {
    modules: Modules,
    decoder: I,
    bus: u8,
    cw: ControlWord,
    output: Vec<(String, String)>,
}

impl Default for BreadboardState {
    fn default() -> Self {
        let a = Register::new(
            "A Register",
            ControlFlag::ARegisterIn,
            ControlFlag::ARegisterOut,
        );
        let b = Register::new_ro("B Register", ControlFlag::BRegisterIn);
        let alu = Alu::new(a.share(), b.share());
        let output = OutputRegister(0);
        let address_register = Register::new_ro("Memory Address", ControlFlag::MemoryAddressIn);
        let mut ram = Ram::new(address_register.share());
        write_sample_program(&mut ram.memory);
        let instruction_register = InstructionRegister::default();
        let instruction = instruction_register.share();
        let program_counter = ProgramCounter(0);
        let decoder =
            BranchingInstructionDecoder::new(instruction, alu.share_carry(), alu.share_zero());
        let decoder_step = DecoderStep(decoder.share_counter());
        let modules: Modules = vec![
            Box::new(program_counter),
            Box::new(address_register),
            Box::new(ram),
            Box::new(instruction_register),
            Box::new(decoder_step),
            Box::new(a),
            Box::new(alu),
            Box::new(b),
            Box::new(output),
            Box::new(EmptyModule),
        ];
        Self::new(modules, decoder)
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

    pub fn update(&mut self) {
        if self.cw.has(ControlFlag::Hlt) {
            return;
        }
        for module in self.modules.iter_mut() {
            module.step(self.cw, self.bus);
        }

        for module in self.modules.iter_mut() {
            module.bus_read(self.cw, self.bus);
        }

        self.decoder.step();
        if self.cw.has(ControlFlag::NextInstruction) {
            self.decoder.reset_counter();
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

fn write_sample_program(ram: &mut [u8; 16]) {
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
