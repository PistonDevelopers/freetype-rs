
use std;
use std::num::FromPrimitive;
use ffi;
use {
    Bitmap,
    FtResult,
    Glyph,
    GlyphMetrics,
    Vector,
};

pub struct GlyphSlot {
    raw: ffi::FT_GlyphSlot,
}

impl GlyphSlot {
    pub fn from_raw(raw: ffi::FT_GlyphSlot) -> GlyphSlot {
        GlyphSlot {
            raw: raw,
        }
    }

    pub fn render_glyph(&self, render_mode: ffi::FT_Render_Mode) -> FtResult<()> {
        unsafe {
            let err = ffi::FT_Render_Glyph(self.raw, render_mode);
            if err == ffi::FT_Err_Ok {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn get_subglyph_info(&self, sub_index: ffi::FT_UInt) -> FtResult<(ffi::FT_Int, ffi::FT_UInt, ffi::FT_Int, ffi::FT_Int, ffi::FT_Matrix)> {
        unsafe {
            let mut index = 0;
            let mut flags = 0;
            let mut arg1 = 0;
            let mut arg2 = 0;
            let mut transfrom = ffi::FT_Matrix {
                xx: 0, xy: 0,
                yx: 0, yy: 0,
            };
            let err = ffi::FT_Get_SubGlyph_Info(self.raw, sub_index, &mut index, &mut flags, &mut arg1, &mut arg2, &mut transfrom);
            if err == ffi::FT_Err_Ok {
                Ok((index, flags, arg1, arg2, transfrom))
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn get_glyph(&self) -> FtResult<Glyph> {
        unsafe {
            let mut aglyph = std::ptr::mut_null();
            let err= ffi::FT_Get_Glyph(self.raw, &mut aglyph);
            if err == ffi::FT_Err_Ok {
                Ok(Glyph::from_raw(aglyph))
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    #[inline(always)]
    pub fn bitmap(&self) -> Bitmap {
        unsafe {
            Bitmap::from_raw(&(*self.raw).bitmap)
        }
    }

    #[inline(always)]
    pub fn bitmap_left(&self) -> i32 {
        unsafe {
            (*self.raw).bitmap_left
        }
    }

    #[inline(always)]
    pub fn bitmap_top(&self) -> i32 {
        unsafe {
            (*self.raw).bitmap_top
        }
    }

    #[inline(always)]
    pub fn advance(&self) -> Vector {
        unsafe {
            (*self.raw).advance
        }
    }

    #[inline(always)]
    pub fn linear_hori_advance(&self) -> ffi::FT_Fixed {
        unsafe {
            (*self.raw).linearHoriAdvance
        }
    }

    #[inline(always)]
    pub fn linear_vert_advance(&self) -> ffi::FT_Fixed {
        unsafe {
            (*self.raw).linearVertAdvance
        }
    }

    #[inline(always)]
    pub fn metrics(&self) -> GlyphMetrics {
        unsafe {
            (*self.raw).metrics
        }
    }

    #[inline(always)]
    pub fn raw(&self) -> &ffi::FT_GlyphSlotRec {
        unsafe {
            &*self.raw
        }
    }
}

