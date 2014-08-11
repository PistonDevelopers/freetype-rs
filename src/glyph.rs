
use std;
use std::num::FromPrimitive;
use ffi;
use {
    BBox,
    BitmapGlyph,
    FtResult,
    Matrix,
    RenderMode,
    Vector,
};

pub struct Glyph {
    raw: ffi::FT_Glyph,
}

impl Glyph {
    pub fn from_raw(raw: ffi::FT_Glyph) -> Glyph {
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

            let err = ffi::FT_Glyph_Transform(self.raw, p_matrix, p_delta);
            if err == ffi::FT_Err_Ok {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn get_cbox(&self, bbox_mode: ffi::FT_Glyph_BBox_Mode) -> BBox {
        unsafe {
            let acbox = ffi::FT_BBox {
                xMin: 0,
                yMin: 0,
                xMax: 0,
                yMax: 0,
            };
            ffi::FT_Glyph_Get_CBox(self.raw, bbox_mode, &acbox);
            acbox
        }
    }

    pub fn to_bitmap(&self, render_mode: RenderMode, origin: Option<Vector>) -> FtResult<BitmapGlyph> {
        unsafe {
            let mut p_origin = std::ptr::null();
            if origin.is_some() {
                p_origin = origin.get_ref() as *const Vector;
            }

            let the_glyph = self.raw;
            let err = ffi::FT_Glyph_To_Bitmap(&the_glyph, render_mode as u32, p_origin, 0);
            if err == ffi::FT_Err_Ok {
                Ok(BitmapGlyph::from_raw(the_glyph as ffi::FT_BitmapGlyph))
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    #[inline(always)]
    pub fn format(&self) -> ffi::FT_Glyph_Format {
        unsafe {
            (*self.raw).format
        }
    }

    #[inline(always)]
    pub fn raw(&self) -> &ffi::FT_GlyphRec {
        unsafe {
            &*self.raw
        }
    }
}

impl Clone for Glyph {
    fn clone(&self) -> Glyph {
        unsafe {
            let mut target = std::ptr::mut_null();
            let err = ffi::FT_Glyph_Copy(self.raw, &mut target);
            if err != ffi::FT_Err_Ok {
                std::io::println(format!("Failed to copy glyph. Error Code: {}", err).as_slice());
            }
            Glyph::from_raw(target)
        }
    }
}

impl Drop for Glyph {
    fn drop(&mut self) {
        unsafe {
            ffi::FT_Done_Glyph(self.raw)
        }
    }
}

