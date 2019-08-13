use crate::modules::{EmptyModule, Module};
#[cfg(feature = "piston")]
pub mod piston;
#[cfg(feature = "piston")]
pub use piston::*;
#[cfg(not(feature = "piston"))]
pub mod sdl;
#[cfg(not(feature = "piston"))]
pub use sdl::*;

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

pub enum VisualRepresentation {
    Text(String),
    LedN(usize, u8, LedColor),
    LedByte(u8, LedColor),
    LedHalf(u8, LedColor),
    LedSplit(u8, LedColor, LedColor),
    Empty,
}

pub trait GraphicalModule: Module {
    fn representation(&self) -> VisualRepresentation;
}

impl GraphicalModule for EmptyModule {
    fn representation(&self) -> VisualRepresentation {
        VisualRepresentation::Empty
    }
}

impl Default for LedColor {
    fn default() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }
}

impl VisualRepresentation {
    pub fn led(byte: u8) -> VisualRepresentation {
        VisualRepresentation::LedByte(byte, LedColor::default())
    }

    #[cfg(feature = "piston")]
    pub fn display(self, graphics: &mut GraphicsState<'_, G2d<'_>>, transform: [[f64; 3]; 2]) {
        use VisualRepresentation::*;
        match self {
            Text(s) => {
                graphics.write(&s, transform);
            }
            LedN(value, num_bits, color) => {
                graphics.draw_leds(value, num_bits, color, transform, None);
            }
            LedByte(byte, color) => {
                graphics.draw_leds(byte as usize, 8, color, transform, None);
            }
            LedHalf(byte, color) => {
                graphics.draw_leds(byte as usize, 4, color, transform, None);
            }
            LedSplit(byte, color1, color2) => {
                let lsb_transform =
                    graphics.draw_leds((byte >> 4) as usize, 4, color1, transform, None);
                graphics.draw_leds(byte as usize, 4, color2, lsb_transform, None);
            }
            Empty => (),
        }
    }

    #[cfg(not(feature = "piston"))]
    pub fn display(self, graphics: &mut GraphicsState<'_>, x: i32, y: i32) -> Result<(), String> {
        use VisualRepresentation::*;
        match self {
            Text(s) => {
                graphics.write(&s, x, y)?;
            }
            LedN(value, num_bits, color) => {
                graphics.draw_leds(value, num_bits, color, (x, y), None)?;
            }
            LedByte(byte, color) => {
                graphics.draw_leds(byte as usize, 8, color, (x, y), None)?;
            }
            LedHalf(byte, color) => {
                graphics.draw_leds(byte as usize, 4, color, (x, y), None)?;
            }
            LedSplit(byte, color1, color2) => {
                let lsb_pos = graphics.draw_leds((byte >> 4) as usize, 4, color1, (x, y), None)?;
                graphics.draw_leds(byte as usize, 4, color2, lsb_pos, None)?;
            }
            Empty => (),
        }
        Ok(())
    }
}

pub const MODULE_WIDTH: usize = 170;
pub const MODULE_HEIGHT: usize = 60;
pub const BUS_WIDTH: usize = 130;
pub const CONTROL_HEIGHT: usize = 80;

const FONT_DATA: &[u8] = include_bytes!("../assets/FiraSans-Regular.ttf");
const FONT_SIZE: u32 = 40;

pub const CW_LABELS: [&str; 32] = [
    "HLT", "MI", "RI", "RO", "IO", "II", "AI", "AO", "ΣO", "SU", "BI", "OI", "CE", "CO", "J",
    "FI", "NI", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
];
