use std::ptr::null_mut;
use {
    ffi,
    Bitmap,
    FtResult,
    Glyph,
    GlyphMetrics,
    Outline,
    RenderMode,
    Vector
};

/// A description of a given subglyph returned by `GlyphSlot::get_subglyph_info`
/// function.
#[derive(Copy, Clone)]
pub struct SubGlyphInfo {
    /// The glyph index of the subglyph.
    pub index: i32,
    /// The subglyph flags, see FT_SUBGLYPH_FLAG_XXX.
    pub flags: u32,
    /// The subglyph's first argument (if any).
    pub arg1: i32,
    /// The subglyph's second argument (if any).
    pub arg2: i32,
    /// The subglyph transformation (if any).
    pub transfrom: ffi::FT_Matrix
}

impl Default for SubGlyphInfo {
    fn default() -> Self {
        SubGlyphInfo {
            index: 0,
            flags: 0,
            arg1: 0,
            arg2: 0,
            transfrom: ffi::FT_Matrix { xx: 0, xy: 0, yx: 0, yy: 0 }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct GlyphSlot {
    library_raw: ffi::FT_Library,
    raw: ffi::FT_GlyphSlot
}

impl GlyphSlot {
    pub fn from_raw(library_raw: ffi::FT_Library, raw: ffi::FT_GlyphSlot) -> Self {
        GlyphSlot {
            library_raw: library_raw,
            raw: raw
        }
    }

    pub fn render_glyph(&self, render_mode: RenderMode) -> FtResult<()> {
        let err = unsafe {
            ffi::FT_Render_Glyph(self.raw, render_mode as u32)
        };
        if err == ffi::FT_Err_Ok {
            Ok(())
        } else {
            Err(err.into())
        }
    }

    pub fn get_subglyph_info(&self, sub_index: u32) -> FtResult<SubGlyphInfo> {
        let mut info = SubGlyphInfo::default();
        let err = unsafe {
            ffi::FT_Get_SubGlyph_Info(self.raw, sub_index, &mut info.index, &mut info.flags,
                                      &mut info.arg1, &mut info.arg2, &mut info.transfrom)
        };
        if err == ffi::FT_Err_Ok {
            Ok(info)
        } else {
            Err(err.into())
        }
    }

    pub fn get_glyph(&self) -> FtResult<Glyph> {
        let mut aglyph = null_mut();

        let err = unsafe {
            ffi::FT_Get_Glyph(self.raw, &mut aglyph)
        };
        if err == ffi::FT_Err_Ok {
            Ok(Glyph::from_raw(self.library_raw, aglyph))
        } else {
            Err(err.into())
        }
    }

    pub fn outline(&self) -> Option<Outline> {
        let outline = unsafe { &(*self.raw).outline };
        let format = unsafe { (*self.raw).format };

        if format == ffi::FT_GLYPH_FORMAT_OUTLINE {
            let outline = unsafe {
                Outline::from_raw(outline)
            };
            Some(outline)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn bitmap(&self) -> Bitmap {
        let bitmap = unsafe { &(*self.raw).bitmap };

        Bitmap::from_raw(bitmap)
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
