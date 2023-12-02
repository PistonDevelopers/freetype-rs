extern crate freetype as ft;
extern crate unicode_normalization;

use unicode_normalization::UnicodeNormalization;

const WIDTH: i32 = 32;
const HEIGHT: i32 = 24;

type Figure = [[u8; WIDTH as usize]; HEIGHT as usize];

fn draw_bitmap(bitmap: ft::Bitmap, x: i32, y: i32) -> Figure {
    let mut figure = [[0; WIDTH as usize]; HEIGHT as usize];
    let w = bitmap.width() as usize;
    let x_max = x + w as i32;
    let y_max = y + bitmap.rows();

    for (p, i) in (x..x_max).enumerate() {
        for (q, j) in (y..y_max).enumerate() {
            if i < 0 || j < 0 || i >= WIDTH || j >= HEIGHT {
                continue;
            }
            figure[j as usize][i as usize] |= bitmap.buffer()[q * w + p];
        }
    }
    figure
}

fn main() {
    let mut args = std::env::args();
    if args.len() != 3 {
        let exe = args.next().unwrap();
        println!("Usage: {} font character", exe);
        return;
    }

    let font = args.nth(1).unwrap();
    let character = args.next().and_then(|s| s.nfc().next()).unwrap() as usize;
    let library = ft::Library::init().unwrap();
    let face = library.new_face(font, 0).unwrap();

    face.set_char_size(40 * 64, 0, 50, 0).unwrap();
    face.load_char(character, ft::face::LoadFlag::RENDER)
        .unwrap();

    let glyph = face.glyph();
    let x = glyph.bitmap_left();
    let y = HEIGHT - glyph.bitmap_top();
    let figure = draw_bitmap(glyph.bitmap(), x, y);
    for row in figure {
        for v in row {
            let c = match v {
                0 => " ",
                1..=127 => "*",
                _ => "+",
            };
            print!("{}", c)
        }
        println!(" ");
    }
}
