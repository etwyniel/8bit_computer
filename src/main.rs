pub mod graphics;
pub mod modules;
pub mod shareable;
pub mod state;
pub mod utils;

use graphics::*;
use modules::*;
use piston_window::*;
use state::BreadboardState;

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

fn interactive_loop<I>(mut state: BreadboardState<I>)
where
    I: InstructionDecoder,
{
    let n_modules = state.modules().len();
    let mut window = init_window(n_modules);
    let glyphs = &mut load_font(&mut window);
    state.pre_step();
    let mut manual = true;
    let mut changed = false;
    window.events.set_lazy(manual);
    window.events.set_ups(60);
    let mut clock_divider = 2;
    let mut cycle_number = 0;
    while let Some(e) = window.next() {
        if let (Some(_), true) = (e.update_args(), changed) {
            changed = false;
            state.pre_step();
        }

        window.draw_2d(&e, |c, g, device| {
            clear([0.75, 0.73, 0.7, 1.0], g);
            let mut graphics = GraphicsState::new(c, g, glyphs);
            graphics.draw_lines(n_modules);
            graphics.display_modules(&state.modules());
            graphics.display_bus(state.bus());
            graphics.display_bus(state.bus());
            graphics.display_cw(state.cw(), n_modules);
            glyphs.factory.encoder.flush(device);
        });

        e.update(|_| {
            cycle_number += 1;
            if !manual && cycle_number % fibo(clock_divider) == 0 {
                cycle_number = 0;
                changed = true;
                state.update();
            }
        });

        e.press(|button| {
            if let Button::Keyboard(key) = button {
                match key {
                    Key::Return if manual => {
                        state.update();
                        state.pre_step();
                    }
                    Key::R => {
                        changed = true;
                        state.reset();
                        state.pre_step();
                    }
                    Key::C => {
                        manual = !manual;
                        window.events.set_lazy(manual);
                    }
                    Key::PageUp => {
                        clock_divider = if clock_divider == 2 {
                            2
                        } else {
                            clock_divider - 1
                        };
                    }
                    Key::PageDown => {
                        clock_divider += 1;
                    }
                    _ => (),
                }
            }
        });
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

fn main() {
    let breadboard = BreadboardState::default();
    interactive_loop(breadboard);
}
