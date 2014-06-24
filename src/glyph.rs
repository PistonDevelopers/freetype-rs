
use std;
use std::num::FromPrimitive;
use ffi::*;
use {
    BBox,
    FtResult,
    Matrix,
    Vector,
};

pub struct Glyph {
    raw: FT_Glyph,
}

impl Glyph {
    pub fn new(raw: FT_Glyph) -> Glyph {
        Glyph {
            raw: raw,
        }
    }

    pub fn transform(&self, matrix: &Matrix, delta: &Vector) -> FtResult<()> {
        unsafe {
            let err = FT_Glyph_Transform(self.raw, matrix, delta);
            if err == FT_Err_Ok {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn get_cbox(&self, bbox_mode: FT_Glyph_BBox_Mode) -> BBox {
        unsafe {
            let acbox = FT_BBox {
                xMin: 0,
                yMin: 0,
                xMax: 0,
                yMax: 0,
            };
            FT_Glyph_Get_CBox(self.raw, bbox_mode, &acbox);
            acbox
        }
    }

    pub fn to_bitmap(&self, render_mode: FT_Render_Mode, origin: Option<Vector>) -> FtResult<Glyph> {
        unsafe {
            let the_glyph = self.raw;
            let err;
            if origin.is_none() {
                err = FT_Glyph_To_Bitmap(&the_glyph, render_mode, std::ptr::null(), 0);
            } else {
                err = FT_Glyph_To_Bitmap(&the_glyph, render_mode, origin.get_ref(), 0);
            }
            if err == FT_Err_Ok {
                Ok(Glyph::new(the_glyph))
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    #[inline(always)]
    pub fn raw<'a>(&'a self) -> &'a FT_GlyphRec {
        unsafe {
            &*self.raw
        }
    }
}

impl Clone for Glyph {
    fn clone(&self) -> Glyph {
        unsafe {
            let target: FT_Glyph = std::ptr::null();
            let err = FT_Glyph_Copy(self.raw, &target);
            if err != FT_Err_Ok {
                std::io::println(format!("Failed to copy glyph. Error Code: {}", err).as_slice());
            }
            Glyph::new(target)
        }
    }
}

impl Drop for Glyph {
    fn drop(&mut self) {
        unsafe {
            FT_Done_Glyph(self.raw)
        }
    }
}

