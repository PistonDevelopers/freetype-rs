
use std;
use std::num::FromPrimitive;
use ffi::*;
use {
    Bitmap,
    FtResult,
    Glyph,
    GlyphMetrics,
    Vector,
};

pub struct GlyphSlot {
    raw: FT_GlyphSlot,
}

impl GlyphSlot {
    pub fn from_raw(raw: FT_GlyphSlot) -> GlyphSlot {
        GlyphSlot {
            raw: raw,
        }
    }

    pub fn render_glyph(&self, render_mode: FT_Render_Mode) -> FtResult<()> {
        unsafe {
            let err = FT_Render_Glyph(self.raw(), render_mode);
            if err == 0 {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn get_subglyph_info(&self, sub_index: FT_UInt) -> FtResult<(FT_Int, FT_UInt, FT_Int, FT_Int, FT_Matrix)> {
        unsafe {
            let index = 0;
            let flags = 0;
            let arg1 = 0;
            let arg2 = 0;
            let transfrom = FT_Matrix {
                xx: 0, xy: 0,
                yx: 0, yy: 0,
            };
            let err = FT_Get_SubGlyph_Info(self.raw(), sub_index, &index, &flags, &arg1, &arg2, &transfrom);
            if err == 0 {
                Ok((index, flags, arg1, arg2, transfrom))
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn get_glyph(&self) -> FtResult<Glyph> {
        unsafe {
            let aglyph: FT_Glyph = std::ptr::null();
            let err= FT_Get_Glyph(self.raw, &aglyph);
            if err == FT_Err_Ok {
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
    pub fn linear_hori_advance(&self) -> FT_Fixed {
        unsafe {
            (*self.raw).linearHoriAdvance
        }
    }

    #[inline(always)]
    pub fn linear_vert_advance(&self) -> FT_Fixed {
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
    pub fn raw<'a>(&'a self) -> &'a FT_GlyphSlotRec {
        unsafe {
            &*self.raw
        }
    }
}

