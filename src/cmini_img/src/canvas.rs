use std::fs::File;
use std::io::BufWriter;
use png::Encoder;
use rusttype::{Font, Point, Scale};

pub type Rgb = (u8, u8, u8);

pub struct RgbCanvas {
    inner: Vec<Rgb>,
    width: usize,
    height: usize,
}

impl RgbCanvas {
    pub fn with_dim(width: usize, height: usize) -> Self {
        const WHITE: Rgb = (u8::MAX, u8::MAX, u8::MAX);
        let inner = vec![WHITE; width * height];
        Self { inner, width, height }
    }
    pub fn with_dim_and_color(width: usize, height: usize, color: Rgb) -> Self {
        let inner = vec![color; width * height];
        Self { inner, width, height }
    }
    pub fn xy_to_index(&self, xy: (usize, usize)) -> usize {
        xy.1 * self.width + xy.0
    }
    pub fn get_pixel(&self, xy: (usize, usize)) -> Option<Rgb> {
        let index = self.xy_to_index(xy);
        self.inner.get(index).copied()
    }
    #[track_caller]
    pub fn set_pixel(&mut self, xy: (usize, usize), color: Rgb) {
        let index = self.xy_to_index(xy);
        self.inner[index] = color;
    }
    pub fn try_set_pixel(&mut self, xy: (usize, usize), color: Rgb) {
        let index = self.xy_to_index(xy);
        if let Some(old_color) = self.inner.get_mut(index) {
            *old_color = color;
        }
    }
    pub fn as_bytes(&self) -> &[u8] {
        let byte_ptr = self.inner.as_ptr() as *const u8;
        let len = self.inner.len() * 3;
        // SAFETY: Vec<Rgb> with n pixels has the same layout as [u8; n * 3]
        unsafe { std::slice::from_raw_parts(byte_ptr, len) }
    }
    pub fn draw<Sh: DrawOn>(&mut self, shape: &Sh, color: Rgb) {
        shape.draw_on(self, color)
    }
    pub fn draw_text(&mut self, text: &str, xy: (usize, usize), font: &Font, font_size: f32, color: Rgb) {
        let scale = Scale::uniform(font_size);
        let v_metrics = font.v_metrics(scale);
        let (r, g, b) = (color.0 as f32, color.1 as f32, color.2 as f32);

        let layout = font.layout(text, scale, Point { x: xy.0 as f32, y: xy.1 as f32 });
        for glyph in layout {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let pt = (
                        (x as i32 + bounding_box.min.x) as usize,
                        (y as i32 + bounding_box.min.y) as usize,
                    );
                    if let Some(old_color) = self.get_pixel(pt) {
                        let old_r = old_color.0 as f32;
                        let old_g = old_color.1 as f32;
                        let old_b = old_color.2 as f32;
                        let inv_v = 1.0 - v;
                        let color = (
                            (old_r * inv_v + r * v) as u8,
                            (old_g * inv_v + g * v) as u8,
                            (old_b * inv_v + b * v) as u8,
                        );
                        self.set_pixel(pt, color)
                    }
                })
            }
        }
    }
    pub fn save(&self, path: &str) -> Result<(), png::EncodingError> {
        let file = File::create(path)?;
        let ref mut w = BufWriter::new(file);

        let mut encoder = Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(self.as_bytes())
    }
}

pub trait DrawOn {
    fn draw_on(&self, canvas: &mut RgbCanvas, color: Rgb);
}