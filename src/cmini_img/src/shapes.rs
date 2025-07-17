use png::Encoder;
use std::fs::File;
use std::io::{BufWriter, Read};
use rusttype::Font;
use crate::canvas::{DrawOn, Rgb, RgbCanvas};

#[derive(Copy, Clone)]
struct Circle {
    x: usize,
    y: usize,
    radius: usize,
}

impl Circle {
    pub fn new(center_xy: (usize, usize), radius: usize) -> Self {
        Self {
            x: center_xy.0,
            y: center_xy.1,
            radius,
        }
    }
}

impl DrawOn for Circle {
    fn draw_on(&self, canvas: &mut RgbCanvas, color: Rgb) {
        // Define bounding box
        let (x_start, y_start) = (self.x.saturating_sub(self.radius), self.y.saturating_sub(self.radius));
        let (x_end, y_end) = (self.x + self.radius, self.y + self.radius);
        let r = self.radius as f32;

        for x in x_start..x_end {
            for y in y_start..y_end {
                let (dx, dy) = ((x as f32 + 0.5) - self.x as f32, (y as f32 + 0.5) - self.y as f32);
                let dist = dx.hypot(dy);
                if dist <= self.radius as f32 {
                    canvas.try_set_pixel((x, y), color);
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
struct Rect {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Rect {
    pub fn new(xy: (usize, usize), width: usize, height: usize) -> Self {
        Self {
            x: xy.0,
            y: xy.1,
            width,
            height,
        }
    }
}

impl DrawOn for Rect {
    fn draw_on(&self, canvas: &mut RgbCanvas, color: Rgb) {
        let (x_end, y_end) = (self.x + self.width, self.y + self.height);
        for x in self.x..x_end {
            for y in self.y..y_end {
                canvas.try_set_pixel((x, y), color);
            }
        }
    }
}

pub(crate) struct RoundedRect {
    rects: [Rect; 5],
    circles: [Circle; 4],
}

impl RoundedRect {
    #[track_caller]
    pub fn new(xy: (usize, usize), width: usize, height: usize, border_radius: usize) -> Self {
        let (dx, dy, dr) = (width, height, border_radius);
        let inner_width = width.checked_sub(2 * dr).unwrap();
        let inner_height = height.checked_sub(2 * dr).unwrap();

        // x0, y0, x1, y1 are the 4 corners of the whole rounded rectangle
        let (x0, y0) = xy;
        let (x1, y1) = (x0 + dx, y0 + dy);
        let (inner_x0, inner_y0) = (x0 + dr, y0 + dr);
        // this sub operation will not overflow, as it just added dx,
        // which is at least twice of dr (checked by checked_sub)
        let (inner_x1, inner_y1) = (x1 - dr, y1 - dr);

        let n_rect = Rect::new((inner_x0, y0), inner_width, border_radius);
        let s_rect = Rect::new((inner_x0, inner_y1), inner_width, border_radius);
        let w_rect = Rect::new((x0, inner_y0), border_radius, inner_height);
        let e_rect = Rect::new((inner_x1, inner_y0), border_radius, inner_height);
        let c_rect = Rect::new((inner_x0, inner_y0), inner_width, inner_height);
        let rects = [n_rect, s_rect, w_rect, e_rect, c_rect];

        let circles = [
            Circle::new((inner_x0, inner_y0), border_radius),
            Circle::new((inner_x1, inner_y0), border_radius),
            Circle::new((inner_x0, inner_y1), border_radius),
            Circle::new((inner_x1, inner_y1), border_radius),
        ];

        Self { rects, circles }
    }
}

impl DrawOn for RoundedRect {
    fn draw_on(&self, canvas: &mut RgbCanvas, color: Rgb) {
        for rect in self.rects {
            canvas.draw(&rect, color);
        }
        for circle in self.circles {
            canvas.draw(&circle, color);
        }
    }
}

#[test]
#[ignore]
fn test() {
    // Create a 200x200 canvas with gray background (RGB: 128, 128, 128)
    let (width, height) = (200, 200);
    let mut canvas = RgbCanvas::with_dim_and_color(width, height, (128, 128, 128));

    // Draw a black circle at center (100, 100) with radius 50
    let circle = Circle::new((40, 40), 20);
    canvas.draw(&circle, (0, 0, 0)); // Black

    // Draw a white rectangle at (50, 50) with width 100 and height 50
    let rect = Rect::new((100, 15), 30, 50);
    canvas.draw(&rect, (255, 255, 255)); // White

    let rounded_rect = RoundedRect::new((15, 120), 60, 30, 5);
    canvas.draw(&rounded_rect, (240, 240, 0));

    // let mut font_bytes = Vec::new();
    // let mut font_file = File::open("Fira Code Regular 400.ttf").unwrap();
    // font_file.read(&mut font_bytes);
    // assert_eq!(font_bytes.len(), 227848);
    let font_bytes = include_bytes!("../Fira Code Regular 400.ttf");
    let font = Font::try_from_bytes(font_bytes.as_ref()).unwrap();
    canvas.draw_text("test", (20, 100), &font, 48.0, (0, 0, 0));

    canvas.save("test_output.png").unwrap();

    // // Save to PNG
    // let path = "test_output.png";
    // let file = File::create(path).unwrap();
    // let ref mut w = BufWriter::new(file);
    //
    // let mut encoder = Encoder::new(w, width as u32, height as u32);
    // encoder.set_color(png::ColorType::Rgb);
    // encoder.set_depth(png::BitDepth::Eight);
    //
    // let mut writer = encoder.write_header().unwrap();
    // writer.write_image_data(canvas.as_bytes()).unwrap();
}