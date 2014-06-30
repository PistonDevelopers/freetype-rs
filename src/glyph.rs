
use std;
use std::num::FromPrimitive;
use ffi::{
    FT_BBox,
    FT_BitmapGlyph,
    FT_Done_Glyph,
    FT_Err_Ok,
    FT_Glyph_To_Bitmap,
    FT_Glyph_BBox_Mode,
    FT_Glyph_Format,
    FT_Glyph,
    FT_Glyph_Copy,
    FT_GlyphRec,
    FT_Glyph_Get_CBox,
    FT_Glyph_Transform,
    FT_Render_Mode,
};
use {
    BBox,
    BitmapGlyph,
    FtResult,
    Matrix,
    Vector,
};

pub struct Glyph {
    raw: FT_Glyph,
}

impl Glyph {
    pub fn from_raw(raw: FT_Glyph) -> Glyph {
        Glyph {
            raw: raw,
        }
    }

    pub fn transform(&self, matrix: Option<Matrix>, delta: Option<Vector>) -> FtResult<()> {
        unsafe {
            let mut p_matrix : *const Matrix = std::ptr::null();
            let mut p_delta : *const Vector = std::ptr::null();

            if matrix.is_some() {
                p_matrix = &matrix.unwrap() as *const Matrix;
            }

            if delta.is_some() {
                p_delta = &delta.unwrap() as *const Vector;
            }

            let err = FT_Glyph_Transform(self.raw, p_matrix, p_delta);
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

    pub fn to_bitmap(&self, render_mode: FT_Render_Mode, origin: Option<Vector>) -> FtResult<BitmapGlyph> {
        unsafe {
            let mut p_origin = std::ptr::null();
            if origin.is_some() {
                p_origin = origin.get_ref() as *const Vector;
            }

            let the_glyph = self.raw;
            let err = FT_Glyph_To_Bitmap(&the_glyph, render_mode, p_origin, 0);
            if err == FT_Err_Ok {
                Ok(BitmapGlyph::from_raw(the_glyph as FT_BitmapGlyph))
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    #[inline(always)]
    pub fn format(&self) -> FT_Glyph_Format {
        unsafe {
            (*self.raw).format
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
            Glyph::from_raw(target)
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

