pub mod graphics;
pub mod modules;
pub mod shareable;
pub mod state;

use clap::{App, Arg};
use graphics::*;
use modules::*;
use state::{write_sample_program, BreadboardState};
use std::time::{Duration, Instant};

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
fn interactive_loop_piston<I>(mut state: BreadboardState<I>) -> Result<(), String>
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
                state.rising_edge();
                state.falling_edge();
            }
        });

        e.press(|button| {
            if let Button::Keyboard(key) = button {
                match key {
                    Key::Return if manual => {
                        state.rising_edge();
                        state.falling_edge();
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
                state.rising_edge();
                state.falling_edge();
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

fn interactive_loop_sdl<I>(mut state: BreadboardState<I>) -> Result<(), String>
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
    let frame_duration = Duration::new(1, 0).checked_div(60).unwrap();
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
        let last_render = Instant::now();
        if !manual {
            cycle_number += 1;
            if cycle_number % fibo(clock_divider) == 0 {
                changed = true;
                cycle_number = 0;
                state.rising_edge();
                state.falling_edge();
            }
        }
        while last_render.elapsed() < frame_duration {
            let event = if manual {
                event_pump.wait_event()
            } else {
                match event_pump.wait_event_timeout(frame_duration.as_millis() as u32) {
                    Some(e) => e,
                    None => {
                        continue;
                    }
                }
            };
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

#[cfg(feature = "piston")]
const DEFAULT_INTERACTIVE_LOOP: &'static str = "piston";
#[cfg(not(feature = "piston"))]
const DEFAULT_INTERACTIVE_LOOP: &'static str = "sdl";

#[cfg(feature = "piston")]
fn interactive_loop<I: InstructionDecoder>(backend: &str, state: BreadboardState<I>) -> Result<(), String> {
    match backend {
        "piston" => interactive_loop_piston(state),
        "sdl" => interactive_loop_sdl(state),
        _ => {
            eprintln!("Unknown rendering backend {}", name);
            std::process::exit(1);
        }
    }
}

#[cfg(not(feature = "piston"))]
fn interactive_loop<I: InstructionDecoder>(backend: &str, state: BreadboardState<I>) -> Result<(), String> {
    if backend == "piston" {
        eprintln!("Error: The program was not compiled with piston enabled.");
        eprintln!("To enable piston, recompile with `--features \"piston\"`");
        std::process::exit(1);
    }
    if backend == "sdl" {
        interactive_loop_sdl(state)
    } else {
        eprintln!("Unknown rendering backend {}", backend);
        std::process::exit(1);
    }
}

fn main() {
    let matches = App::new("8bit computer")
        .version("0.1.1")
        .author("Aymeric Beringer <aymeric@beringer.cf>")
        .arg(
            Arg::with_name("microcode")
                .short("m")
                .long("microcode")
                .value_name("FILE")
                .help("Use microcode from file instead of predefined logic")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("backend")
                .short("b")
                .long("backend")
                .value_name("BACKEND")
                .possible_values(&["sdl", "piston"])
                .help(concat!("Select the graphics backend if compiled with ",
                              "piston enabled, piston is the default.")),
        ).get_matches();
    let backend = matches.value_of("backend").unwrap_or(DEFAULT_INTERACTIVE_LOOP);
    if let Some(microcode_filename) = matches.value_of("microcode") {
        let breadboard =
            match BreadboardState::from_microcode(microcode_filename, write_sample_program) {
                Err(s) => {
                    eprintln!("Could not open microcode file: {}", s);
                    std::process::exit(1);
                }
                Ok(state) => state,
            };
        interactive_loop(backend, breadboard).unwrap();
    } else {
        let breadboard = BreadboardState::default_with_ram(write_sample_program);
        if let Err(s) = interactive_loop(backend, breadboard) {
            eprintln!("Error: {}", s);
            std::process::exit(1);
        }
    }
}
