#![allow(unstable)]

extern crate freetype;

use freetype as ft;
use freetype::ffi;

const WIDTH: usize = 32;
const HEIGHT: usize = 24;

fn draw_bitmap(bitmap: &ffi::FT_Bitmap, x: usize, y: usize) ->
    [[u8; WIDTH]; HEIGHT]
{
    let mut image = [[0u8; WIDTH]; HEIGHT];
    let mut p = 0;
    let mut q = 0;
    let x_max = x + bitmap.width as usize;
    let y_max = y + bitmap.rows as usize;

    for i in range(x, x_max) {
        for j in range(y, y_max) {
            if i >= WIDTH || j >= HEIGHT {
                continue;
            }

            unsafe {
                image[j][i] |= *bitmap.buffer.offset((q * bitmap.width + p) as isize);
            }
            q += 1;
        }
        q = 0;
        p += 1;
    }

    image
}

fn main() {
    let mut stderr = &mut std::io::stderr();
    let args = std::os::args_as_bytes();
    if args.len() != 3 {
        let exe = String::from_utf8_lossy(args[0].as_slice());
        let _ = writeln!(stderr, "Usage: {} font sample-text", exe);
        std::os::set_exit_status(1);
        return;
    }

    let filename = args[1].as_slice();
    let text = String::from_utf8_lossy(args[2].as_slice());

    let library = ft::Library::init().unwrap();
    let face = library.new_face(&Path::new(filename), 0).unwrap();
    face.set_char_size(40 * 64, 0, 50, 0).unwrap();
    face.load_char(text.nfc_chars().next().unwrap() as usize, ft::face::RENDER).unwrap();

    let slot = face.glyph().raw();

    let x = slot.bitmap_left as usize;
    let y = HEIGHT - slot.bitmap_top as usize;
    let image = draw_bitmap(&slot.bitmap, x, y);

    for i in range(0, HEIGHT) {
        for j in range(0, WIDTH) {
            std::io::print(if image[i as usize][j as usize] == 0 {
                               " "
                           } else if image[i as usize][j as usize] < 128 {
                               "*"
                           } else {
                               "+"
                           });
        }
        std::io::println("");
    }

}
