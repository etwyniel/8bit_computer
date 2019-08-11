mod modules;
pub mod shareable;

use modules::*;
use std::io::BufRead;

pub fn run_cycle<I>(modules: &mut [Box<dyn Module>], decoder: &mut I) -> bool
where
    I: InstructionDecoder,
{
    let cw = decoder.decode();
    decoder.step();
    if cw.has(ControlFlag::Hlt) {
        return false;
    }
    eprintln!("cw: {}", cw);
    for module in modules.iter_mut() {
        module.pre_step(cw);
    }
    let mut bus = None;
    for module in modules.iter_mut() {
        bus = bus.or_else(|| module.bus_write(cw));
    }
    let bus = bus.unwrap_or(0);
    for module in modules.iter_mut() {
        module.step(cw, bus);
    }

    for module in modules.iter_mut() {
        module.bus_read(cw, bus);
    }
    // for module in modules.iter() {
    //     eprintln!("module: {:?}", module);
    // }
    dbg!(bus);
    true
}

fn pretty_print_output(lines: &[(String, String)]) {
    let longest = lines.iter().map(|(name, _)| name.len()).max().unwrap_or(0);
    for line in lines {
        println!("\x1b[1;32m{:>width$}\x1b[0m {}", &line.0, &line.1, width = longest);
    }
}

fn interactive_loop<I>(mut modules: Vec<Box<dyn Module>>, mut decoder: I)
where
    I: InstructionDecoder,
{
    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    let mut line = String::new();
    let mut output = Vec::with_capacity(modules.len() + 2);
    loop {
        println!("Decoding step n°{}", decoder.get_counter());
        let mut cw = decoder.decode();
        decoder.step();
        for module in modules.iter_mut() {
            module.pre_step(cw);
        }
        let mut bus = None;
        for module in modules.iter_mut() {
            bus = bus.or_else(|| module.bus_write(cw));
        }
        let bus = bus.unwrap_or(0);

        output.clear();
        for module in modules.iter() {
            output.push((module.get_name().to_string(), format!("{}", module)));
        }
        output.push(("Control word".to_string(), format!("{}", cw)));
        output.push(("Bus".to_string(), format!("{:08b}", bus)));
        pretty_print_output(&output);

        for module in modules.iter_mut() {
            module.step(cw, bus);
        }

        line.clear();
        input.read_line(&mut line).unwrap();
        match line.trim().as_ref() {
            "q" | "quit" => break,
            "reset" => {
                for module in modules.iter_mut() {
                    module.reset();
                }
                cw = ControlWord(0);
            }
            _ => (),
        }

        if cw.has(ControlFlag::Hlt) {
            break;
        }

        for module in modules.iter_mut() {
            module.bus_read(cw, bus);
        }

        if cw.has(ControlFlag::NextInstruction) {
            decoder.reset_counter();
        }
    }
}

fn write_program(ram: &mut [u8; 16]) {
    ram[0x0] = 0x1e; // LDA 14
    ram[0x1] = 0x2f; // ADD 15
    ram[0x2] = 0xe0; // OUT
    ram[0x3] = 0xf0; // HLT
    ram[0xe] = 14;
    ram[0xf] = 28;
}

fn main() {
    let a = Register::new(
        "A Register",
        ControlFlag::ARegisterIn,
        ControlFlag::ARegisterOut,
    );
    let b = Register::new_ro("B Register", ControlFlag::BRegisterIn);
    let alu = Alu::new(a.share(), b.share());
    let output = OutputRegister(0);
    let address_register = Register::new_ro("Address", ControlFlag::MemoryAddressIn);
    let mut ram = Ram::new(address_register.share());
    write_program(&mut ram.memory);
    let instruction_register = InstructionRegister::default();
    let instruction = instruction_register.share();
    let program_counter = ProgramCounter(0);
    let decoder =
        BranchingInstructionDecoder::new(instruction, alu.share_carry(), alu.share_zero());
    let modules: Vec<Box<dyn Module>> = vec![
        Box::new(program_counter),
        Box::new(instruction_register),
        Box::new(address_register),
        Box::new(ram),
        Box::new(a),
        Box::new(b),
        Box::new(alu),
        Box::new(output),
    ];
    interactive_loop(modules, decoder);
    // loop {
    //     if !run_cycle(&mut modules, &mut decoder) {
    //         break;
    //     }
    // }
}
