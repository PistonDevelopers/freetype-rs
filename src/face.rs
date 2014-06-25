
use std;
use std::num::FromPrimitive;
use ffi::*;
use {
    Library,
    FtResult,
    Matrix,
    Vector,
    GlyphSlot,
};

pub struct Face {
    raw: FT_Face,
    glyph: GlyphSlot,
}

impl Face {
    pub fn new(library: &Library, filepathname: &str, face_index: FT_Long) -> FtResult<Face> {
        unsafe {
            let face: FT_Face = std::ptr::null();
            let err = FT_New_Face(library.raw(), filepathname.as_slice().as_ptr(), face_index, &face);
            if err == 0 {
                Ok(Face {
                    raw: face,
                    glyph: GlyphSlot::new((*face).glyph),
                })
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn new_memory(library: &Library, buffer: &[u8], face_index: FT_Long) -> FtResult<Face> {
        unsafe {
            let face: FT_Face = std::ptr::null();
            let err = FT_New_Memory_Face(library.raw(), buffer.as_ptr(), buffer.len() as i64, face_index, &face);
            if err == 0 {
                Ok(Face {
                    raw: face,
                    glyph: GlyphSlot::new((*face).glyph),
                })
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn attach_file(&self, filepathname: &str) -> FtResult<()> {
        unsafe {
            let err = FT_Attach_File(self.raw(), filepathname.as_slice().as_ptr() as *i8);
            if err == 0 {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn reference(&self) -> FtResult<()> {
        unsafe {
            let err = FT_Reference_Face(self.raw());
            if err == 0 {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn set_char_size(&self, char_width: FT_F26Dot6, char_height: FT_F26Dot6, horz_resolution: FT_UInt, vert_resolution: FT_UInt) -> FtResult<()> {
        unsafe {
            let err = FT_Set_Char_Size(self.raw(), char_width, char_height, horz_resolution, vert_resolution);
            if err == 0 {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn load_glyph(&self, glyph_index: FT_UInt, load_flags: LoadFlag) -> FtResult<()> {
        unsafe {
            let err = FT_Load_Glyph(self.raw(), glyph_index, load_flags.bits);
            if err == 0 {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn load_char(&self, char_code: FT_ULong, load_flags: LoadFlag) -> FtResult<()> {
        unsafe {
            let err = FT_Load_Char(self.raw(), char_code, load_flags.bits);
            if err == 0 {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn set_transform(&self, matrix: &Matrix, delta: &Vector) {
        unsafe {
            FT_Set_Transform(self.raw(), matrix, delta);
        }
    }

    pub fn get_char_index(&self, charcode: FT_ULong) -> FT_UInt {
        unsafe {
            FT_Get_Char_Index(self.raw, charcode)
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
        FT_HAS_HORIZONTAL(self.raw)
    }

    #[inline(always)]
    pub fn has_vertical(&self) -> bool {
        FT_HAS_VERTICAL(self.raw)
    }

    #[inline(always)]
    pub fn has_kerning(&self) -> bool {
        FT_HAS_KERNING(self.raw)
    }

    #[inline(always)]
    pub fn is_scalable(&self) -> bool {
        FT_IS_SCALABLE(self.raw)
    }

    #[inline(always)]
    pub fn is_sfnt(&self) -> bool {
        FT_IS_SFNT(self.raw)
    }

    #[inline(always)]
    pub fn is_fixed_width(&self) -> bool {
        FT_IS_FIXED_WIDTH(self.raw)
    }

    #[inline(always)]
    pub fn has_fixed_sizes(&self) -> bool {
        FT_HAS_FIXED_SIZES(self.raw)
    }

    #[inline(always)]
    pub fn has_glyph_names(&self) -> bool {
        FT_HAS_GLYPH_NAMES(self.raw)
    }

    #[inline(always)]
    pub fn is_cid_keyed(&self) -> bool {
        FT_IS_CID_KEYED(self.raw)
    }

    #[inline(always)]
    pub fn is_tricky(&self) -> bool {
        FT_IS_TRICKY(self.raw)
    }

    #[inline(always)]
    pub fn has_color(&self) -> bool {
        FT_HAS_COLOR(self.raw)
    }

    #[inline(always)]
    pub fn raw<'a>(&'a self) -> &'a FT_FaceRec {
        unsafe {
            &*self.raw
        }
    }
}

impl Drop for Face {
    fn drop(&mut self) {
        unsafe {
            let err = FT_Done_Face(self.raw());
            if err != 0 {
                std::io::println(format!("Failed to drop face. Error Code: {}", err).as_slice());
            }
        }
    }
}

bitflags!(flags LoadFlag: i32 {
    static Default = FT_LOAD_DEFAULT,
    static NoScale = FT_LOAD_NO_SCALE,
    static NoHinting = FT_LOAD_NO_HINTING,
    static Render = FT_LOAD_RENDER,
    static NoBitmap = FT_LOAD_NO_BITMAP,
    static VerticalLayout = FT_LOAD_VERTICAL_LAYOUT,
    static ForceAutohint = FT_LOAD_FORCE_AUTOHINT,
    static CropBitmap = FT_LOAD_CROP_BITMAP,
    static Pendantic = FT_LOAD_PENDANTIC,
    static IgnoreGlobalAdvanceWidth = FT_LOAD_IGNORE_GLOBAL_ADVANCE_WIDTH,
    static NoRecurse = FT_LOAD_NO_RECURSE,
    static IgnoreTransform = FT_LOAD_IGNORE_TRANSFORM,
    static Monochrome = FT_LOAD_MONOCHROME,
    static LinearDesign = FT_LOAD_LINEAR_DESIGN,
    static NoAutohint = FT_LOAD_NO_AUTOHINT,
    static Color = FT_LOAD_COLOR
})

