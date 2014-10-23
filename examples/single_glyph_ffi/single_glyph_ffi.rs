
extern crate freetype;

use freetype::ffi;

const WIDTH: ffi::FT_Int = 32;
const HEIGHT: ffi::FT_Int = 24;

fn draw_bitmap(bitmap: &ffi::FT_Bitmap, x: ffi::FT_Int, y: ffi::FT_Int) -> [[u8, ..WIDTH as uint], ..HEIGHT as uint] {
    let mut image: [[u8, ..WIDTH as uint], ..HEIGHT as uint] = [
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    ];
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
    unsafe {
        let args = std::os::args();
        if args.len() != 3 {
            println!("usage: {} font sample-text\n", &args[0]);
            return;
        }

        let filename = &args[1];
        let text = &args[2];

        let mut library: ffi::FT_Library = std::ptr::null_mut();
        let error = ffi::FT_Init_FreeType(&mut library);
        if error != ffi::FT_Err_Ok {
            println!("Could not initialize freetype.");
            return;
        }

        let mut face: ffi::FT_Face = std::ptr::null_mut();
        let error = ffi::FT_New_Face(library, filename.to_c_str().as_ptr(), 0, &mut face);
        if error != ffi::FT_Err_Ok {
            println!("Could not load font '{}'. Error Code: {}", filename, error);
            return;
        }

        ffi::FT_Set_Char_Size(face, 40 * 64, 0, 50, 0);
        let slot = &*(*face).glyph;

        let error = ffi::FT_Load_Char(face, text.as_bytes()[0] as u64, ffi::FT_LOAD_RENDER);
        if error != ffi::FT_Err_Ok {
            println!("Could not load char.");
            return;
        }

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

        ffi::FT_Done_Face(face);
        ffi::FT_Done_FreeType(library);
    }
}

