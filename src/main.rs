mod modules;

use modules::*;

pub fn run_cycle<I>(modules: &mut [Box<dyn Module>], decoder: &mut I) -> bool
where
    I: InstructionDecoder,
{
    let cw = decoder.decode();
    decoder.step();
    if cw.has(ControlFlag::Hlt) {
        return false;
    }
    eprintln!("cw: 0b{:016b}", cw.0);
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

fn main() {
    let a = Register::new(ControlFlag::ARegisterIn, ControlFlag::ARegisterOut);
    let b = Register::new_ro(ControlFlag::BRegisterIn);
    let alu = Alu::new(a.get_ref(), b.get_ref());
    let output = OutputRegister(0);
    let address_register = Register::new_ro(ControlFlag::MemoryAddressIn);
    let mut ram = Ram::new(address_register.get_ref());
    ram.memory[0x0] = 0x1e; // LDA 14
    ram.memory[0x1] = 0x2f; // ADD 15
    ram.memory[0x2] = 0xe0; // OUT
    ram.memory[0x3] = 0xf0; // HLT
    ram.memory[0xe] = 14;
    ram.memory[0xf] = 28;
    let instruction_register = InstructionRegister::default();
    let instruction = instruction_register.get_ref();
    let program_counter = ProgramCounter(0);
    let mut decoder = SimpleInstructionDecoder::new(instruction);
    let mut modules: Vec<Box<dyn Module>> = vec![
        Box::new(a),
        Box::new(b),
        Box::new(alu),
        Box::new(output),
        Box::new(ram),
        Box::new(address_register),
        Box::new(instruction_register),
        Box::new(program_counter),
    ];
    loop {
        if !run_cycle(&mut modules, &mut decoder) {
            break;
        }
    }
}
