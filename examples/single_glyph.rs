#![feature(core, exit_status, collections)]

extern crate "freetype" as ft;

use std::io::prelude::*;

use std::io::stderr;
use std::path::Path;
use std::env::{ args, set_exit_status };
use ft::ffi;

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
    let ref mut stderr = stderr();
    let ref mut args = args();
    if let (3, _) = args.size_hint() {}
    else {
        let exe = args.next().unwrap();
        let _ = writeln!(stderr, "Usage: {} font sample-text", exe);
        set_exit_status(1);
        return
    }

    let filename = args.nth(1).unwrap();
    let text = args.next().unwrap();

    let library = ft::Library::init().unwrap();
    let face = library.new_face(&Path::new(&filename), 0).unwrap();
    face.set_char_size(40 * 64, 0, 50, 0).unwrap();
    face.load_char(text.nfc_chars().next().unwrap() as usize, ft::face::RENDER).unwrap();

    let slot = face.glyph().raw();

    let x = slot.bitmap_left as usize;
    let y = HEIGHT - slot.bitmap_top as usize;
    let image = draw_bitmap(&slot.bitmap, x, y);

    for i in range(0, HEIGHT) {
        for j in range(0, WIDTH) {
            print!("{}", if image[i as usize][j as usize] == 0 {
                             " "
                         } else if image[i as usize][j as usize] < 128 {
                             "*"
                         } else {
                             "+"
                         });
        }
        println!("");
    }
}
