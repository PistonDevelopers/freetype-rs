
use std;
use std::num::FromPrimitive;
use ffi;
use {
    FtResult,
    Matrix,
    Vector,
    GlyphSlot,
};

pub struct Face {
    raw: ffi::FT_Face,
    glyph: GlyphSlot,
}

impl Face {
    pub fn from_raw(raw: ffi::FT_Face) -> Face {
        Face {
            raw: raw,
            glyph: unsafe { GlyphSlot::from_raw((*raw).glyph) },
        }
    }

    pub fn attach_file(&self, filepathname: &str) -> FtResult<()> {
        unsafe {
            let err = ffi::FT_Attach_File(self.raw, filepathname.as_slice().as_ptr() as *const i8);
            if err == ffi::FT_Err_Ok {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn reference(&self) -> FtResult<()> {
        unsafe {
            let err = ffi::FT_Reference_Face(self.raw);
            if err == ffi::FT_Err_Ok {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn set_char_size(&self, char_width: ffi::FT_F26Dot6, char_height: ffi::FT_F26Dot6, horz_resolution: ffi::FT_UInt, vert_resolution: ffi::FT_UInt) -> FtResult<()> {
        unsafe {
            let err = ffi::FT_Set_Char_Size(self.raw, char_width, char_height, horz_resolution, vert_resolution);
            if err == ffi::FT_Err_Ok {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn set_pixel_sizes(&self, pixel_width: ffi::FT_UInt, pixel_height: ffi::FT_UInt) -> FtResult<()> {
        unsafe {
            let err = ffi::FT_Set_Pixel_Sizes(self.raw, pixel_width, pixel_height);
            if err == ffi::FT_Err_Ok {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn load_glyph(&self, glyph_index: ffi::FT_UInt, load_flags: LoadFlag) -> FtResult<()> {
        unsafe {
            let err = ffi::FT_Load_Glyph(self.raw, glyph_index, load_flags.bits);
            if err == ffi::FT_Err_Ok {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn load_char(&self, char_code: ffi::FT_ULong, load_flags: LoadFlag) -> FtResult<()> {
        unsafe {
            let err = ffi::FT_Load_Char(self.raw, char_code, load_flags.bits);
            if err == ffi::FT_Err_Ok {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn set_transform(&self, matrix: &Matrix, delta: &Vector) {
        unsafe {
            ffi::FT_Set_Transform(self.raw, matrix, delta);
        }
    }

    pub fn get_char_index(&self, charcode: ffi::FT_ULong) -> ffi::FT_UInt {
        unsafe {
            ffi::FT_Get_Char_Index(self.raw, charcode)
        }
    }

    // According to FreeType doc, each time you load a new glyph image,
    // the previous one is erased from the glyph slot.
    #[inline(always)]
    pub fn glyph<'a>(&'a self) -> &'a GlyphSlot {
        &self.glyph
    }

    #[inline(always)]
    pub fn has_horizontal(&self) -> bool {
        ffi::FT_HAS_HORIZONTAL(self.raw)
    }

    #[inline(always)]
    pub fn has_vertical(&self) -> bool {
        ffi::FT_HAS_VERTICAL(self.raw)
    }

    #[inline(always)]
    pub fn has_kerning(&self) -> bool {
        ffi::FT_HAS_KERNING(self.raw)
    }

    #[inline(always)]
    pub fn is_scalable(&self) -> bool {
        ffi::FT_IS_SCALABLE(self.raw)
    }

    #[inline(always)]
    pub fn is_sfnt(&self) -> bool {
        ffi::FT_IS_SFNT(self.raw)
    }

    #[inline(always)]
    pub fn is_fixed_width(&self) -> bool {
        ffi::FT_IS_FIXED_WIDTH(self.raw)
    }

    #[inline(always)]
    pub fn has_fixed_sizes(&self) -> bool {
        ffi::FT_HAS_FIXED_SIZES(self.raw)
    }

    #[inline(always)]
    pub fn has_glyph_names(&self) -> bool {
        ffi::FT_HAS_GLYPH_NAMES(self.raw)
    }

    #[inline(always)]
    pub fn is_cid_keyed(&self) -> bool {
        ffi::FT_IS_CID_KEYED(self.raw)
    }

    #[inline(always)]
    pub fn is_tricky(&self) -> bool {
        ffi::FT_IS_TRICKY(self.raw)
    }

    #[inline(always)]
    pub fn has_color(&self) -> bool {
        ffi::FT_HAS_COLOR(self.raw)
    }

    #[inline(always)]
    pub fn raw<'a>(&'a self) -> &'a ffi::FT_FaceRec {
        unsafe {
            &*self.raw
        }
    }
}

impl Drop for Face {
    fn drop(&mut self) {
        unsafe {
            let err = ffi::FT_Done_Face(self.raw);
            if err != 0 {
                std::io::println(format!("Failed to drop face. Error Code: {}", err).as_slice());
            }
        }
    }
}

bitflags!(flags LoadFlag: i32 {
    static Default = ffi::FT_LOAD_DEFAULT,
    static NoScale = ffi::FT_LOAD_NO_SCALE,
    static NoHinting = ffi::FT_LOAD_NO_HINTING,
    static Render = ffi::FT_LOAD_RENDER,
    static NoBitmap = ffi::FT_LOAD_NO_BITMAP,
    static VerticalLayout = ffi::FT_LOAD_VERTICAL_LAYOUT,
    static ForceAutohint = ffi::FT_LOAD_FORCE_AUTOHINT,
    static CropBitmap = ffi::FT_LOAD_CROP_BITMAP,
    static Pendantic = ffi::FT_LOAD_PENDANTIC,
    static IgnoreGlobalAdvanceWidth = ffi::FT_LOAD_IGNORE_GLOBAL_ADVANCE_WIDTH,
    static NoRecurse = ffi::FT_LOAD_NO_RECURSE,
    static IgnoreTransform = ffi::FT_LOAD_IGNORE_TRANSFORM,
    static Monochrome = ffi::FT_LOAD_MONOCHROME,
    static LinearDesign = ffi::FT_LOAD_LINEAR_DESIGN,
    static NoAutohint = ffi::FT_LOAD_NO_AUTOHINT,
    static Color = ffi::FT_LOAD_COLOR
})

