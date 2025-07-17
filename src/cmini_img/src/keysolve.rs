use rusttype::Font;
use crate::canvas::{Rgb, RgbCanvas};
use crate::shapes::RoundedRect;

const COL_SPACE: usize = 90;
const ROW_SPACE: usize = 90;
const KEY_SIZE: usize = 80;
const KEY_BORDER_RADIUS: usize = 5;

#[derive(Copy, Clone)]
#[repr(usize)]
enum Percentile {
    Top20 = 0,
    Top40,
    Mid,
    Bottom40,
    Bottom20,
}

impl Percentile {
    pub fn color(self, colors: &[Rgb; 5]) -> Rgb {
        colors[self as usize]
    }
}

#[test]
fn gen_img() {
    use Percentile as P;
    let keys = [
        ('a', (0u8, 0u8, P::Top20)),
        ('b', (0, 1, P::Bottom40)),
        ('c', (0, 2, P::Mid)),
        ('d', (0, 3, P::Top40)),
        ('e', (0, 4, P::Top20)),
        ('f', (0, 5, P::Mid)),
        ('g', (1, 0, P::Mid)),
        ('h', (1, 1, P::Top20)),
        ('i', (1, 2, P::Top20)),
        ('j', (1, 3, P::Bottom20)),
    ].into_iter();
    let colors = [
        (0xff, 0x85, 0x85),
        (0xd7, 0x74, 0x9d),
        (0x9e, 0x6d, 0xa4),
        (0x63, 0x66, 0x96),
        (0x35, 0x59, 0x78),
    ];

    let font_bytes = include_bytes!("../Fira Code Regular 400.ttf");
    let font = Font::try_from_bytes(font_bytes.as_ref()).unwrap();

    let mut img = RgbCanvas::with_dim_and_color(900, 450, (0x27, 0x27, 0x27));
    let background = RoundedRect::new((15, 15), 870, 420, 4);
    let background_color = (0x3e, 0x3e, 0x3e);

    img.draw(&background, background_color);

    for (char, (row, col, percentile)) in keys {
        let (row, col) = (row as usize, col as usize);
        let key_rect = RoundedRect::new((col * COL_SPACE + 20, row *  ROW_SPACE + 20), KEY_SIZE, KEY_SIZE, KEY_BORDER_RADIUS);
        img.draw(&key_rect, percentile.color(&colors));

        let s = char.to_uppercase().to_string();
        img.draw_text(&s, (col * COL_SPACE + 50, row * ROW_SPACE + 70), &font, 48.0, (0x27, 0x27, 0x27));
    }
    img.save("rounded_rectangle.png").unwrap();
}