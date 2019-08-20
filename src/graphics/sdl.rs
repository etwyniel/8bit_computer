use super::*;
use crate::control::ControlWord;
use sdl2::gfx::primitives::DrawRenderer;
pub use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureQuery},
    rwops::RWops,
    ttf::{Font, Sdl2TtfContext},
    video::Window,
    VideoSubsystem,
};

pub struct GraphicsState<'a> {
    pub canvas: Canvas<Window>,
    pub font: Font<'a, 'static>,
    n_lines: i32,
}

impl<'a> GraphicsState<'a> {
    pub fn new(
        video: &VideoSubsystem,
        n_modules: usize,
        ttf_ctx: &'a Sdl2TtfContext,
    ) -> Result<Self, String> {
        let window = init_window(video, n_modules)?;
        let canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;
        let font = load_font(ttf_ctx)?;
        let n_lines = (n_modules / 2 + n_modules % 2) as i32;
        Ok(GraphicsState {
            canvas,
            font,
            n_lines,
        })
    }

    pub fn write(&mut self, data: &str, x: i32, y: i32) -> Result<(), String> {
        if data.is_empty() {
            return Ok(());
        }
        let texture_creator = self.canvas.texture_creator();
        let surface = self
            .font
            .render(data)
            .blended((0, 0, 0))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let TextureQuery { width, height, .. } = texture.query();
        self.canvas
            .copy(&texture, None, Some((x, y, width, height).into()))
    }

    pub fn draw_lines(&mut self) -> Result<(), String> {
        let m_height = MODULE_HEIGHT as i32;
        let m_width = MODULE_WIDTH as i32;
        let b_width = BUS_WIDTH as i32;
        let bus_left = m_width;
        let bus_right = bus_left + b_width;
        let right = bus_right + m_width;
        let bus_height = m_height * self.n_lines;
        self.canvas.set_draw_color((0, 0, 0));
        self.canvas.draw_line((0, 0), (m_width, 0))?;
        self.canvas.draw_line((bus_left, 0), (bus_right, 0))?;
        self.canvas
            .draw_line((bus_left, bus_height), (bus_right, bus_height))?;
        self.canvas
            .draw_line((bus_left, 0), (bus_left, bus_height))?;
        self.canvas
            .draw_line((bus_right, 0), (bus_right, bus_height))?;
        self.canvas.draw_line((0, 0), (0, bus_height))?;
        self.canvas.draw_line((right, 0), (right, bus_height))?;
        for lnum in 0..=self.n_lines {
            let y = m_height * lnum;
            self.canvas.draw_line((0, y), (bus_left, y))?;
            self.canvas.draw_line((bus_right, y), (right, y))?;
        }
        Ok(())
    }

    pub fn draw_dot(&mut self, x: i32, y: i32, (r, g, b): (u8, u8, u8)) -> Result<(), String> {
        // Hack to have an antialiased filled circle (sort of)
        self.canvas
            .filled_circle(x as i16, y as i16, 5, (r, g, b, 255))?;
        self.canvas
            .aa_circle(x as i16, y as i16, 5, (r, g, b, 255))?;
        self.canvas.aa_circle(x as i16, y as i16, 4, (r, g, b, 255))
    }

    pub fn draw_leds(
        &mut self,
        value: usize,
        num_bits: u8,
        color: LedColor,
        (x, y): (i32, i32),
        labels: Option<&[&str]>,
    ) -> Result<(i32, i32), String> {
        for bit in 1..=num_bits {
            let (x, y) = (x + (i32::from(bit) - 1) * 12 + 5, y);
            let [r, g, b, _] = if (value & (1 << num_bits - bit)) > 0 {
                color.on_color
            } else {
                color.off_color
            };
            self.draw_dot(
                x,
                y,
                ((255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8),
            )?;
            if let Some(labels) = labels {
                let (x, mut y) = (x - 3, y + 6);
                for c in labels[bit as usize - 1].chars() {
                    self.write(&c.to_string(), x, y)?;
                    y += 11;
                }
            }
        }
        Ok((x + i32::from(num_bits) * 12, y))
    }

    pub fn display_bus(&mut self, bus: u8) -> Result<(), String> {
        self.write("Bus", MODULE_WIDTH as i32 + 5, 5)?;
        VisualRepresentation::led(bus).display(self, MODULE_WIDTH as i32 + 5, 35)
    }

    pub fn display_cw(&mut self, cw: ControlWord) -> Result<(), String> {
        let (x, y) = (5, MODULE_HEIGHT as i32 * self.n_lines + 5);
        self.write("Control Word", x, y)?;
        self.draw_leds(
            cw.0.reverse_bits() as usize,
            32,
            LedColor::new(0.3, 0.3, 1.0),
            (x, y + 30),
            Some(&CW_LABELS),
        )?;
        Ok(())
    }

    pub fn display_modules(&mut self, modules: &[Box<dyn GraphicalModule>]) -> Result<(), String> {
        for (index, module) in modules.iter().enumerate() {
            let top_left_y = (index % self.n_lines as usize) * MODULE_HEIGHT;
            let top_left_x = if index >= self.n_lines as usize {
                MODULE_WIDTH + BUS_WIDTH
            } else {
                0
            };
            let (x, y) = (top_left_x as i32 + 5, top_left_y as i32 + 5);
            self.write(module.get_name(), x, y)?;
            module.representation().display(self, x, y + 30)?;
        }
        Ok(())
    }
}

pub fn load_font(context: &Sdl2TtfContext) -> Result<Font<'_, 'static>, String> {
    let rwops = RWops::from_bytes(FONT_DATA)?;
    context.load_font_from_rwops(rwops, FONT_SIZE as u16 / 3)
}

pub fn init_window(video: &VideoSubsystem, n_modules: usize) -> Result<Window, String> {
    let width = 2 * MODULE_WIDTH + BUS_WIDTH;
    let height = (n_modules / 2 + n_modules % 2) * MODULE_HEIGHT + CONTROL_HEIGHT;
    video
        .window("8bit computer", width as u32, height as u32)
        .build()
        .map_err(|e| e.to_string())
}
