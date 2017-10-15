extern crate freetype as ft;
extern crate unicode_normalization;

use unicode_normalization::UnicodeNormalization;

const WIDTH: usize = 32;
const HEIGHT: usize = 24;

fn draw_bitmap(bitmap: ft::Bitmap, x: usize, y: usize) -> [[u8; WIDTH]; HEIGHT] {
    let mut figure = [[0; WIDTH]; HEIGHT];
    let mut p = 0;
    let mut q = 0;
    let w = bitmap.width() as usize;
    let x_max = x + w;
    let y_max = y + bitmap.rows() as usize;

    for i in x .. x_max {
        for j in y .. y_max {
            if i < WIDTH && j < HEIGHT {
                figure[j][i] |= bitmap.buffer()[q * w + p];
                q += 1;
            }
        }
        q = 0;
        p += 1;
    }
    figure
}

fn main() {
    let ref mut args = std::env::args();

    if args.len() != 3 {
        let exe = args.next().unwrap();
        println!("Usage: {} font character", exe);
        return
    }

    let ref font = args.nth(1).unwrap();
    let character = args.next().and_then(|s| s.nfc().next()).unwrap() as usize;
    let library = ft::Library::init().unwrap();
    let face = library.new_face(font, 0).unwrap();

    face.set_char_size(40 * 64, 0, 50, 0).unwrap();
    face.load_char(character, ft::face::LoadFlag::RENDER).unwrap();

    let glyph = face.glyph();
    let x = glyph.bitmap_left() as usize;
    let y = HEIGHT - glyph.bitmap_top() as usize;
    let figure = draw_bitmap(glyph.bitmap(), x, y);

    for i in 0 .. HEIGHT {
        for j in 0 .. WIDTH {
            print!("{}",
                match figure[i][j] {
                    p if p == 0 => " ",
                    p if p < 128 => "*",
                    _  => "+"
                }
            );
        }
        println!("");
    }
}
