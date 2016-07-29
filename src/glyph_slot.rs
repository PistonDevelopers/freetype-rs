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

/// A struct encapsulating the space for a glyph within a `Library`
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct GlyphSlot {
    library_raw: ffi::FT_Library,
    raw: ffi::FT_GlyphSlot
}

impl GlyphSlot {
    /// Create a `GlyphSlot` from its constituent C parts
    pub unsafe fn from_raw(library_raw: ffi::FT_Library, raw: ffi::FT_GlyphSlot) -> Self {
        GlyphSlot {
            library_raw: library_raw,
            raw: raw
        }
    }

    /// Convert a given glyph image to a bitmap. It does so by inspecting the glyph image format,
    /// finding the relevant renderer, and invoking it.
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

    /// Retrieve a description of a given subglyph. Only use it if the glyph's format is
    /// FT_GLYPH_FORMAT_COMPOSITE; an error is returned otherwise.
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

    /// Returns a glyph object, that is similar to a `GlyphSlot` but managed outside of the library
    pub fn get_glyph(&self) -> FtResult<Glyph> {
        let mut aglyph = null_mut();

        let err = unsafe {
            ffi::FT_Get_Glyph(self.raw, &mut aglyph)
        };
        if err == ffi::FT_Err_Ok {
            Ok(unsafe { Glyph::from_raw(self.library_raw, aglyph) })
        } else {
            Err(err.into())
        }
    }

    /// In freetype, the `Outline` object is a scalable glyph. This method unpacks a glyph into
    /// this object, or returns `None` if the glyph has no `outline`
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


    /// This field is used as a bitmap descriptor when the slot format is FT_GLYPH_FORMAT_BITMAP.
    /// Note that the address and content of the bitmap buffer can change between calls of
    /// FT_Load_Glyph and a few other functions.
    #[inline(always)]
    pub fn bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_raw(&(*self.raw).bitmap) }
    }

    /// The bitmap's left bearing expressed in integer pixels. Only valid if the format is
    /// FT_GLYPH_FORMAT_BITMAP, this is, if the glyph slot contains a bitmap.
    #[inline(always)]
    pub fn bitmap_left(&self) -> i32 {
        unsafe {
            (*self.raw).bitmap_left
        }
    }

    /// The bitmap's top bearing expressed in integer pixels. Remember that this is the distance
    /// from the baseline to the top-most glyph scanline, upwards y coordinates being positive.
    #[inline(always)]
    pub fn bitmap_top(&self) -> i32 {
        unsafe {
            (*self.raw).bitmap_top
        }
    }

    /// This shorthand is, depending on FT_LOAD_IGNORE_TRANSFORM, the transformed (hinted) advance
    /// width for the glyph, in 26.6 fractional pixel format. As specified with
    /// FT_LOAD_VERTICAL_LAYOUT, it uses either the ‘horiAdvance’ or the ‘vertAdvance’ value of
    /// ‘metrics’ field.
    #[inline(always)]
    pub fn advance(&self) -> Vector {
        unsafe {
            (*self.raw).advance
        }
    }

    /// The advance width of the unhinted glyph. Its value is expressed in 16.16 fractional pixels,
    /// unless FT_LOAD_LINEAR_DESIGN is set when loading the glyph. This field can be important to
    /// perform correct WYSIWYG layout. Only relevant for outline glyphs.
    #[inline(always)]
    pub fn linear_hori_advance(&self) -> ffi::FT_Fixed {
        unsafe {
            (*self.raw).linearHoriAdvance
        }
    }

    /// The advance height of the unhinted glyph. Its value is expressed in 16.16 fractional
    /// pixels, unless FT_LOAD_LINEAR_DESIGN is set when loading the glyph. This field can be
    /// important to perform correct WYSIWYG layout. Only relevant for outline glyphs.
    #[inline(always)]
    pub fn linear_vert_advance(&self) -> ffi::FT_Fixed {
        unsafe {
            (*self.raw).linearVertAdvance
        }
    }

    /// The metrics of the last loaded glyph in the slot. The returned values depend on the last
    /// load flags (see the FT_Load_Glyph API function) and can be expressed either in 26.6
    /// fractional pixels or font units.
    #[inline(always)]
    pub fn metrics(&self) -> GlyphMetrics {
        unsafe {
            (*self.raw).metrics
        }
    }

    /// Get a pointer to the underlying c struct
    #[inline(always)]
    pub fn raw(&self) -> &ffi::FT_GlyphSlotRec {
        unsafe {
            &*self.raw
        }
    }
}
