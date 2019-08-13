pub mod graphics;
pub mod modules;
pub mod shareable;
pub mod state;
pub mod utils;

use graphics::*;
use modules::*;
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

#[cfg(feature = "piston")]
fn interactive_loop<I>(mut state: BreadboardState<I>) -> Result<(), String>
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
    Ok(())
}

#[cfg(not(feature = "piston"))]
fn handle_event<I>(
    state: &mut BreadboardState<I>,
    manual: &mut bool,
    clock_divider: &mut usize,
    event: sdl2::event::Event,
) -> bool
where
    I: InstructionDecoder,
{
    use sdl2::{event::Event, keyboard::*};
    if let Event::KeyDown {
        keycode: Some(key), ..
    } = event
    {
        match key {
            Keycode::Return if *manual => {
                state.update();
                return true;
            }
            Keycode::C => {
                *manual = !*manual;
            }
            Keycode::PageUp if *clock_divider > 2 => {
                *clock_divider -= 1;
            }
            Keycode::PageDown => {
                *clock_divider += 1;
            }
            Keycode::R => {
                state.reset();
                return true;
            }
            _ => (),
        };
    }
    false
}

#[cfg(not(feature = "piston"))]
fn interactive_loop<I>(mut state: BreadboardState<I>) -> Result<(), String>
where
    I: InstructionDecoder,
{
    use sdl2::{event::Event, keyboard::*};
    let n_modules = state.modules().len();
    let ttf_ctx = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut graphics = GraphicsState::new(&video_subsystem, n_modules, &ttf_ctx)?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut manual = true;
    let mut changed = true;
    let mut clock_divider = 2;
    let mut cycle_number = 0;
    loop {
        if changed {
            changed = false;
            state.pre_step();
        }
        graphics.canvas.set_draw_color((191, 186, 179));
        graphics.canvas.clear();
        graphics.draw_lines()?;
        graphics.display_modules(state.modules())?;
        graphics.display_bus(state.bus())?;
        graphics.display_cw(state.cw())?;
        graphics.canvas.present();
        if !manual {
            cycle_number += 1;
            if cycle_number % fibo(clock_divider) == 0 {
                changed = true;
                cycle_number = 0;
                state.update();
            }
        }
        for event in event_pump.poll_iter() {
            if let Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            }
            | Event::Quit { .. } = event
            {
                return Ok(());
            }
            changed |= handle_event(&mut state, &mut manual, &mut clock_divider, event);
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

fn main() {
    let breadboard = BreadboardState::default();
    if let Err(s) = interactive_loop(breadboard) {
        eprintln!("Error: {}", s);
        std::process::exit(1);
    }
}
