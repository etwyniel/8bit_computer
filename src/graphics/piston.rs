use super::*;
use crate::modules::control::*;
use gfx_graphics::{TextureContext, TextureSettings};
pub use piston_window::*;
use rusttype::Font;

pub struct GraphicsState<'a, G: Graphics> {
    pub g: &'a mut G,
    pub glyphs: &'a mut Glyphs,
    pub transform: [[f64; 3]; 2],
}

impl<'a, 'b: 'a> GraphicsState<'a, G2d<'b>> {
    pub fn new(c: Context, g: &'a mut G2d<'b>, glyphs: &'a mut Glyphs) -> Self {
        GraphicsState {
            g,
            glyphs,
            transform: c.transform,
        }
    }

    pub fn write(&mut self, data: &str, transform: [[f64; 3]; 2]) {
        text(
            color::BLACK,
            FONT_SIZE,
            data,
            self.glyphs,
            transform.scale_pos([0.3, 0.3]),
            self.g,
        ).unwrap();
    }

    pub fn draw_dot(&mut self, transform: [[f64; 3]; 2], color: [f32; 4]) {
        ellipse(
            color,
            rectangle::square(0.0, 0.0, 10.0),
            transform.trans(0.0, -5.0),
            self.g,
        );
    }

    pub fn draw_leds(
        &mut self,
        value: usize,
        num_bits: u8,
        color: LedColor,
        transform: [[f64; 3]; 2],
        labels: Option<&[&str]>,
    ) -> [[f64; 3]; 2] {
        for bit in 1..=num_bits {
            let transform = transform.trans((f64::from(bit) - 1.0) * 12.0, 0.0);
            let color = if (value & (1 << (num_bits - bit))) > 0 {
                color.on_color
            } else {
                color.off_color
            };
            self.draw_dot(transform, color);
            if let Some(labels) = labels {
                let mut transform = transform.trans(0.0, 18.0);
                for c in labels[bit as usize - 1].chars() {
                    self.write(&c.to_string(), transform);
                    transform = transform.trans(0.0, 12.0);
                }
            }
        }
        transform.trans(f64::from(num_bits) * 12.0, 0.0)
    }

    pub fn display_modules(&mut self, modules: &[Box<dyn GraphicalModule>]) {
        let n_lines = modules.len() / 2 + modules.len() % 2;
        for (index, module) in modules.iter().enumerate() {
            let top_left_y = (index % n_lines) * MODULE_HEIGHT;
            let top_left_x = if index >= n_lines {
                MODULE_WIDTH + BUS_WIDTH
            } else {
                0
            };
            let transform = self
                .transform
                .trans(top_left_x as f64, top_left_y as f64)
                .trans(5.0, 15.0);
            self.write(module.get_name(), transform);
            let contents_transform = transform.trans(0.0, 20.0);
            module.representation().display(self, contents_transform);
        }
    }

    pub fn line(&mut self, l: [f64; 4], x: f64, y: f64) {
        line(color::BLACK, 1.0, l, self.transform.trans(x, y), self.g);
    }

    pub fn draw_lines(&mut self, n_modules: usize) {
        let n_lines = (n_modules / 2) + (n_modules % 2);
        let vline = [0.0, 0.0, 0.0, (n_lines * MODULE_HEIGHT) as f64];
        let hline = [0.0, 0.0, MODULE_WIDTH as f64, 0.0];
        let bus_line = [0.0, 0.0, BUS_WIDTH as f64, 0.0];
        self.line(vline, 0.0, 0.0);
        self.line(vline, MODULE_WIDTH as f64, 0.0);
        self.line(vline, (MODULE_WIDTH + BUS_WIDTH) as f64, 0.0);
        self.line(vline, (MODULE_WIDTH * 2 + BUS_WIDTH) as f64, 0.0);
        self.line(bus_line, MODULE_WIDTH as f64, 0.0);
        self.line(
            bus_line,
            MODULE_WIDTH as f64,
            (MODULE_HEIGHT * n_lines) as f64,
        );
        for lnum in 0..=n_lines {
            self.line(hline, 0.0, (MODULE_HEIGHT * lnum) as f64);
            self.line(
                hline,
                (MODULE_WIDTH + BUS_WIDTH) as f64,
                (MODULE_HEIGHT * lnum) as f64,
            );
        }
    }

    pub fn display_bus(&mut self, bus: u8) {
        let transform = self.transform.trans(MODULE_WIDTH as f64, 0.0);
        self.write("Bus", transform.trans(5.0, 15.0));
        VisualRepresentation::led(bus).display(self, transform.trans(5.0, 35.0));
    }

    pub fn display_cw(&mut self, cw: ControlWord, n_modules: usize) {
        let transform = self.transform.trans(
            0.0,
            (MODULE_HEIGHT * (n_modules / 2 + n_modules % 2)) as f64,
        );
        self.write("Control Word", transform.trans(5.0, 15.0));
        self.draw_leds(
            cw.0.reverse_bits() as usize,
            32,
            LedColor::new(0.3, 0.3, 1.0),
            transform.trans(5.0, 30.0),
            Some(&CW_LABELS),
        );
    }
}

pub fn init_window(n_modules: usize) -> PistonWindow {
    let width = 2 * MODULE_WIDTH + BUS_WIDTH;
    let height = (n_modules / 2 + n_modules % 2) * MODULE_HEIGHT + CONTROL_HEIGHT;
    WindowSettings::new("8bit computer", [width as f64, height as f64])
        .resizable(false)
        .exit_on_esc(true)
        .automatic_close(true)
        .vsync(true)
        .build()
        .unwrap()
}

pub fn load_font(window: &mut PistonWindow) -> Glyphs {
    let font: Font<'static> = Font::from_bytes(FONT_DATA).unwrap();
    Glyphs::from_font(
        font,
        TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into(),
        },
        TextureSettings::new(),
    )
}
