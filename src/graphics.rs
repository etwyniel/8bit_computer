use crate::modules::{ControlWord, Module};
use piston_window::*;

const FONT_SIZE: u32 = 40;

pub fn draw_dot<G: Graphics>(transform: [[f64; 3]; 2], color: [f32; 4], g: &mut G) {
    ellipse(
        color,
        rectangle::square(0.0, 0.0, 10.0),
        transform.trans(0.0, -5.0),
        g,
    );
}

pub fn draw_leds(
    value: usize,
    num_bits: u8,
    color: LedColor,
    transform: [[f64; 3]; 2],
    labels: Option<&[&str]>,
    glyphs: &mut Glyphs,
    g: &mut G2d,
) -> [[f64; 3]; 2] {
    for bit in 1..=num_bits {
        let transform = transform.trans((bit as f64 - 1.0) * 12.0, 0.0);
        let color = if (value & (1 << (num_bits - bit))) > 0 {
            color.on_color
        } else {
            color.off_color
        };
        draw_dot(transform, color, g);
        if let Some(labels) = labels {
            let mut transform = transform.trans(0.0, 18.0);
            for c in labels[(num_bits - bit) as usize].chars() {
                write(&c.to_string(), glyphs, transform, g);
                transform = transform.trans(0.0, 12.0);
            }
        }
    }
    transform.trans(num_bits as f64 * 12.0, 0.0)
}

pub fn write(data: &str, glyphs: &mut Glyphs, transform: [[f64; 3]; 2], g: &mut G2d) {
    text(color::BLACK, FONT_SIZE, data, glyphs, transform.scale_pos([0.3, 0.3]), g).unwrap();
}

#[derive(Copy, Clone)]
pub struct LedColor {
    pub on_color: [f32; 4],
    pub off_color: [f32; 4],
}

impl LedColor {
    pub fn new(r: f32, g: f32, b: f32) -> LedColor {
        LedColor {
            on_color: [r, g, b, 1.0],
            off_color: [r * 0.4, g * 0.4, b * 0.4, 1.0],
        }
    }
}

impl Default for LedColor {
    fn default() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }
}

pub enum VisualRepresentation {
    Text(String),
    LedByte(u8, LedColor),
    LedHalf(u8, LedColor),
    LedSplit(u8, LedColor, LedColor),
}

impl VisualRepresentation {
    pub fn led(byte: u8) -> VisualRepresentation {
        VisualRepresentation::LedByte(byte, LedColor::default())
    }

    pub fn display(self, glyphs: &mut Glyphs, transform: [[f64; 3]; 2], g: &mut G2d) {
        use VisualRepresentation::*;
        match self {
            Text(s) => {
                write(&s, glyphs, transform, g);
            }
            LedByte(byte, color) => {
                draw_leds(byte as usize, 8, color, transform, None, glyphs, g);
            }
            LedHalf(byte, color) => {
                draw_leds(byte as usize, 4, color, transform, None, glyphs, g);
            }
            LedSplit(byte, color1, color2) => {
                let lsb_transform =
                    draw_leds((byte >> 4) as usize, 4, color1, transform, None, glyphs, g);
                draw_leds(byte as usize, 4, color2, lsb_transform, None, glyphs, g);
            }
        }
    }
}

pub trait ModuleGraphic: Module {
    fn representation(&self) -> VisualRepresentation;
}

pub const MODULE_WIDTH: usize = 170;
pub const MODULE_HEIGHT: usize = 60;
pub const BUS_WIDTH: usize = 130;
pub const CONTROL_HEIGHT: usize = 80;

pub fn display_modules(
    modules: &Vec<Box<dyn ModuleGraphic>>,
    glyphs: &mut Glyphs,
    c: Context,
    g: &mut G2d,
) {
    for (index, module) in modules.iter().enumerate() {
        let top_left_y = (index % (modules.len() / 2)) * MODULE_HEIGHT;
        let top_left_x = if index >= modules.len() / 2 {
            MODULE_WIDTH + BUS_WIDTH
        } else {
            0
        };
        let transform = c
            .transform
            .trans(top_left_x as f64, top_left_y as f64)
            .trans(5.0, 15.0);
        write(module.get_name(), glyphs, transform, g);
        let contents_transform = transform.trans(0.0, 20.0);
        module
            .representation()
            .display(glyphs, contents_transform, g);
    }
}

pub fn init_window(n_modules: usize) -> PistonWindow {
    let width = 2 * MODULE_WIDTH + BUS_WIDTH;
    let height = n_modules / 2 * MODULE_HEIGHT + CONTROL_HEIGHT;
    WindowSettings::new("8bit computer", [width as f64, height as f64])
        .resizable(false)
        .exit_on_esc(true)
        .automatic_close(true)
        .vsync(true)
        .build()
        .unwrap()
}

pub fn draw_lines(n_modules: usize, c: Context, g: &mut G2d) {
    macro_rules! line {
        ($l:expr, $x:expr, $y:expr) => {
            line(
                color::BLACK,
                1.0,
                $l,
                c.transform.trans(($x) as f64, ($y) as f64),
                g,
            );
        };
    }
    let vline = [0.0, 0.0, 0.0, ((n_modules / 2) * MODULE_HEIGHT) as f64];
    let hline = [0.0, 0.0, MODULE_WIDTH as f64, 0.0];
    let bus_line = [0.0, 0.0, BUS_WIDTH as f64, 0.0];
    line!(vline, 0.0, 0.0);
    line!(vline, MODULE_WIDTH, 0.0);
    line!(vline, MODULE_WIDTH + BUS_WIDTH, 0.0);
    line!(vline, MODULE_WIDTH * 2 + BUS_WIDTH, 0.0);
    line!(bus_line, MODULE_WIDTH, 0.0);
    line!(bus_line, MODULE_WIDTH, MODULE_HEIGHT * n_modules / 2);
    for lnum in 0..(n_modules / 2 + 1) {
        line!(hline, 0.0, MODULE_HEIGHT * lnum);
        line!(hline, MODULE_WIDTH + BUS_WIDTH, MODULE_HEIGHT * lnum);
    }
}

pub fn display_bus(bus: u8, transform: [[f64; 3]; 2], glyphs: &mut Glyphs, g: &mut G2d) {
    write("Bus", glyphs, transform.trans(5.0, 15.0), g);
    VisualRepresentation::led(bus).display(glyphs, transform.trans(5.0, 35.0), g);
}

pub fn display_cw(cw: ControlWord, transform: [[f64; 3]; 2], glyphs: &mut Glyphs, g: &mut G2d) {
    write("Control Word", glyphs, transform.trans(5.0, 15.0), g);
    const LABELS: [&'static str; 32] = [
        "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "NI", "FI", "J", "CO", "CE",
        "OI", "BI", "SU", "ΣO", "AO", "AI", "II", "IO", "RO", "RI", "MI", "HLT",
    ];
    let (mut reversed, mut copy) = (0, cw.0);
    for _ in 1..=32 {
        reversed = (reversed >> 1) | (copy & (1 << 31));
        copy <<= 1;
    }
    draw_leds(
        reversed as usize,
        32,
        LedColor::new(0.3, 0.3, 1.0),
        transform.trans(5.0, 30.0),
        Some(&LABELS),
        glyphs,
        g,
    );
}