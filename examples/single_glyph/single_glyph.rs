
extern crate freetype;

use freetype as ft;
use freetype::ffi;

const WIDTH: ffi::FT_Int = 32;
const HEIGHT: ffi::FT_Int = 24;

fn draw_bitmap(bitmap: &ffi::FT_Bitmap, x: ffi::FT_Int, y: ffi::FT_Int) -> [[u8, ..WIDTH as uint], ..HEIGHT as uint] {
    let mut image = [[0u8, ..WIDTH as uint], .. HEIGHT as uint];
    let mut p = 0;
    let mut q = 0;
    let x_max = x + bitmap.width;
    let y_max = y + bitmap.rows;

    for i in range(x, x_max) {
        for j in range(y, y_max) {
            if i < 0      || j < 0       ||
               i >= WIDTH || j >= HEIGHT {
                continue;
            }

            unsafe {
                image[j as uint][i as uint] |= *bitmap.buffer.offset((q * bitmap.width + p) as int);
            }
            q += 1;
        }
        q = 0;
        p += 1;
    }

    image
}

fn main() {
    let args = std::os::args();
    if args.len() != 3 {
        println!("usage: {} font sample-text\n", &args[0]);
        return;
    }

    let ref filename = args[1];
    let ref text = args[2];

    let library = ft::Library::init().unwrap();
    let mut face = library.new_face(filename.as_slice(), 0).unwrap();
    face.set_char_size(40 * 64, 0, 50, 0).unwrap();
    face.load_char(text.nfc_chars().next().unwrap() as u64, ft::face::RENDER).unwrap();

    let slot = face.glyph().raw();
    let image = draw_bitmap(&slot.bitmap, slot.bitmap_left, HEIGHT - slot.bitmap_top);

    for i in range(0, HEIGHT) {
        for j in range(0, WIDTH) {
            std::io::print(if image[i as uint][j as uint] == 0 {
                               " "
                           } else if image[i as uint][j as uint] < 128 {
                               "*"
                           } else {
                               "+"
                           });
        }
        std::io::println("");
    }

}
