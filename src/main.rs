pub mod graphics;
mod modules;
pub mod shareable;

use graphics::*;
use modules::*;
use piston_window::*;

type Modules = Vec<Box<dyn ModuleGraphic>>;

#[cfg(unix)]
fn use_color() -> bool {
    use std::os::unix::io::{AsRawFd, RawFd};

    extern "C" {
        fn isatty(fd: RawFd) -> std::os::raw::c_int;
    }
    unsafe { isatty(std::io::stdout().as_raw_fd()) == 1 }
}

#[cfg(not(unix))]
fn use_color() -> bool {
    false
}

fn pretty_print_output(lines: &[(String, String)]) {
    let longest = lines.iter().map(|(name, _)| name.len()).max().unwrap_or(0);
    for (ref name, ref contents) in lines {
        if name.is_empty() && contents.is_empty() {
            continue;
        }
        if use_color() {
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

fn update_state<I>(modules: &mut Modules, cw: ControlWord, bus: u8, decoder: &mut I)
where
    I: InstructionDecoder,
{
    if cw.has(ControlFlag::Hlt) {
        return;
    }
    for module in modules.iter_mut() {
        module.step(cw, bus);
    }

    for module in modules.iter_mut() {
        module.bus_read(cw, bus);
    }

    decoder.step();
    if cw.has(ControlFlag::NextInstruction) {
        decoder.reset_counter();
    }
}

fn pre_step<I: InstructionDecoder>(
    modules: &mut Modules,
    decoder: &mut I,
    output: &mut Vec<(String, String)>,
) -> (ControlWord, u8) {
    let cw = decoder.decode();
    for module in modules.iter_mut() {
        module.pre_step(cw);
    }
    let mut maybe_bus = None;
    for module in modules.iter_mut() {
        maybe_bus = maybe_bus.or_else(|| module.bus_write(cw));
    }
    let bus = maybe_bus.unwrap_or(0);
    output.clear();

    for module in modules.iter() {
        output.push((module.get_name().to_string(), format!("{}", module)));
    }
    output.push(("Control word".to_string(), format!("{}", cw)));
    output.push(("Bus".to_string(), format!("{:08b}", bus)));
    pretty_print_output(&output);
    (cw, bus)
}

fn fibo(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    }
    let (mut x0, mut x1) = (0, 1);
    for _ in 1..n {
        std::mem::swap(&mut x0, &mut x1);
        x1 += x0;
    }
    x1
}

fn interactive_loop<I>(mut modules: Modules, mut decoder: I)
where
    I: InstructionDecoder,
{
    let mut window = init_window(modules.len());
    let ref mut glyphs = load_font(&mut window);
    let mut output = Vec::with_capacity(modules.len() + 2);
    let mut cw = ControlWord(0);
    let mut bus = 0;
    let mut changed = true;
    let mut manual = true;
    let mut clock_divider = 2;
    let mut cycle_number = 0;
    while let Some(e) = window.next() {
        if let (Some(_), true) = (e.update_args(), changed) {
            changed = false;
            let state = pre_step(&mut modules, &mut decoder, &mut output);
            cw = state.0;
            bus = state.1;
        }
        window.draw_2d(&e, |c, g, device| {
            clear([0.75, 0.73, 0.7, 1.0], g);
            draw_lines(modules.len(), c, g);
            display_modules(&modules, glyphs, c, g);
            display_bus(bus, c.transform.trans(MODULE_WIDTH as f64, 0.0), glyphs, g);
            display_cw(
                cw,
                c.transform
                    .trans(0.0, (MODULE_HEIGHT * (modules.len() / 2 + modules.len() % 2)) as f64),
                glyphs,
                g,
            );
            glyphs.factory.encoder.flush(device);
        });
        e.update(|_| {
            cycle_number += 1;
            if !manual && cycle_number % fibo(clock_divider) == 0 {
                cycle_number = 0;
                changed = true;
                update_state(&mut modules, cw, bus, &mut decoder);
            }
        });

        if let (true, Some(Button::Keyboard(Key::Return))) = (manual, e.press_args()) {
            changed = true;
            update_state(&mut modules, cw, bus, &mut decoder);
        } else if let Some(Button::Keyboard(Key::R)) = e.press_args() {
            changed = true;
            for module in modules.iter_mut() {
                module.reset();
            }
            decoder.reset_counter();
        } else if let Some(Button::Keyboard(Key::C)) = e.press_args() {
            manual = !manual;
        } else if let Some(Button::Keyboard(Key::PageUp)) = e.press_args() {
            clock_divider = if clock_divider == 2 { 2 } else { clock_divider - 1 };
        } else if let Some(Button::Keyboard(Key::PageDown)) = e.press_args() {
            clock_divider += 1;
        }
    }
}

#[allow(unused)]
fn write_program(ram: &mut [u8; 16]) {
    ram[0x0] = 0x1e; // LDA 14
    ram[0x1] = 0x2f; // ADD 15
    ram[0x2] = 0xe0; // OUT
    ram[0x3] = 0xf0; // HLT
    ram[0xe] = 14;
    ram[0xf] = 28;
}

fn write_program2(ram: &mut [u8; 16]) {
    ram[0x0] = 0xe0;
    ram[0x1] = 0x2f;
    ram[0x2] = 0x74;
    ram[0x3] = 0x60;
    ram[0x4] = 0x3f;
    ram[0x5] = 0xe0;
    ram[0x6] = 0x80;
    ram[0x7] = 0x64;
    ram[0xf] = 1;
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
    let address_register = Register::new_ro("Memory Address", ControlFlag::MemoryAddressIn);
    let mut ram = Ram::new(address_register.share());
    write_program2(&mut ram.memory);
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
    interactive_loop(modules, decoder);
}
