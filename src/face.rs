use std;
use std::num::FromPrimitive;
use std::ffi::c_str_to_bytes;
use std::str;
use std::borrow::ToOwned;
use ffi;
use {
    FtResult,
    GlyphSlot,
    Matrix,
    Vector,
};

#[repr(u32)]
#[derive(Copy)]
pub enum KerningMode {
    KerningDefault = ffi::FT_KERNING_DEFAULT,
    KerningUnfitted = ffi::FT_KERNING_UNFITTED,
    KerningUnscaled = ffi::FT_KERNING_UNSCALED
}

bitflags!(
flags LoadFlag: i32 {
    const DEFAULT                    = ffi::FT_LOAD_DEFAULT,
    const NO_SCALE                   = ffi::FT_LOAD_NO_SCALE,
    const NO_HINTING                 = ffi::FT_LOAD_NO_HINTING,
    const RENDER                     = ffi::FT_LOAD_RENDER,
    const NO_BITMAP                  = ffi::FT_LOAD_NO_BITMAP,
    const VERTICAL_LAYOUT            = ffi::FT_LOAD_VERTICAL_LAYOUT,
    const FORCE_AUTOHINT             = ffi::FT_LOAD_FORCE_AUTOHINT,
    const CROP_BITMAP                = ffi::FT_LOAD_CROP_BITMAP,
    const PEDANTIC                   = ffi::FT_LOAD_PEDANTIC,
    const IGNORE_GLOBAL_ADVANCE_WITH = ffi::FT_LOAD_IGNORE_GLOBAL_ADVANCE_WIDTH,
    const NO_RECURSE                 = ffi::FT_LOAD_NO_RECURSE,
    const IGNORE_TRANSFORM           = ffi::FT_LOAD_IGNORE_TRANSFORM,
    const MONOCHROME                 = ffi::FT_LOAD_MONOCHROME,
    const LINEAR_DESIGN              = ffi::FT_LOAD_LINEAR_DESIGN,
    const NO_AUTOHINT                = ffi::FT_LOAD_NO_AUTOHINT,
    const TARGET_NORMAL              = ffi::FT_LOAD_TARGET_NORMAL,
    const TARGET_LIGHT               = ffi::FT_LOAD_TARGET_LIGHT,
    const TARGET_MONO                = ffi::FT_LOAD_TARGET_MONO,
    const TARGET_LCD                 = ffi::FT_LOAD_TARGET_LCD,
    const TARGET_LCD_V               = ffi::FT_LOAD_TARGET_LCD_V,
    const COLOR                      = ffi::FT_LOAD_COLOR
});

#[derive(Eq, PartialEq, Hash)]
pub struct Face {
    library_raw: ffi::FT_Library,
    raw: ffi::FT_Face,
    glyph: GlyphSlot,
}

impl Face {
    pub fn from_raw(library_raw: ffi::FT_Library, raw: ffi::FT_Face) -> Face {
        unsafe {
            ffi::FT_Reference_Library(library_raw);
        }

        Face {
            library_raw: library_raw,
            raw: raw,
            glyph: unsafe { GlyphSlot::from_raw(library_raw, (*raw).glyph) },
        }
    }

    pub fn attach_file(&mut self, filepathname: &str) -> FtResult<()> {
        unsafe {
            let err = ffi::FT_Attach_File(self.raw, filepathname.as_slice().as_ptr() as *const i8);
            if err == ffi::FT_Err_Ok {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn reference(&mut self) -> FtResult<()> {
        unsafe {
            let err = ffi::FT_Reference_Face(self.raw);
            if err == ffi::FT_Err_Ok {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn set_char_size(&mut self, char_width: isize, char_height: isize, horz_resolution: u32, vert_resolution: u32) -> FtResult<()> {
        unsafe {
            let err = ffi::FT_Set_Char_Size(self.raw, char_width as ffi::FT_F26Dot6,
                                            char_height as ffi::FT_F26Dot6, horz_resolution,
                                            vert_resolution);
            if err == ffi::FT_Err_Ok {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn set_pixel_sizes(&mut self, pixel_width: u32, pixel_height: u32) -> FtResult<()> {
        unsafe {
            let err = ffi::FT_Set_Pixel_Sizes(self.raw, pixel_width, pixel_height);
            if err == ffi::FT_Err_Ok {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn load_glyph(&mut self, glyph_index: u32, load_flags: LoadFlag) -> FtResult<()> {
        unsafe {
            let err = ffi::FT_Load_Glyph(self.raw, glyph_index, load_flags.bits);
            if err == ffi::FT_Err_Ok {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn load_char(&mut self, char_code: usize, load_flags: LoadFlag) -> FtResult<()> {
        unsafe {
            let err = ffi::FT_Load_Char(self.raw, char_code as ffi::FT_ULong, load_flags.bits);
            if err == ffi::FT_Err_Ok {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn set_transform(&mut self, matrix: &Matrix, delta: &Vector) {
        unsafe {
            ffi::FT_Set_Transform(self.raw, matrix, delta);
        }
    }

    pub fn get_char_index(&self, charcode: usize) -> u32 {
        unsafe {
            ffi::FT_Get_Char_Index(self.raw, charcode as ffi::FT_ULong)
        }
    }

    pub fn get_kerning(&self, left_char_index: u32, right_char_index: u32, kern_mode: KerningMode)
        -> FtResult<Vector> {

        let vec = Vector { x: 0, y: 0 };

        let err_code = unsafe {
            ffi::FT_Get_Kerning(self.raw, left_char_index, right_char_index,
                                kern_mode as u32, std::mem::transmute(&vec))
        };

        if err_code == ffi::FT_Err_Ok {
            Ok(vec)
        } else {
            Err(FromPrimitive::from_i32(err_code).unwrap())
        }
    }

    // According to FreeType doc, each time you load a new glyph image,
    // the previous one is erased from the glyph slot.
    #[inline(always)]
    pub fn glyph(&self) -> &GlyphSlot {
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
    pub fn raw(&self) -> &ffi::FT_FaceRec {
        unsafe {
            &*self.raw
        }
    }

    #[inline(always)]
    pub fn ascender(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).ascender
        }
    }

    #[inline(always)]
    pub fn descender(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).descender
        }
    }

    #[inline(always)]
    pub fn height(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).height
        }
    }

    #[inline(always)]
    pub fn max_advance_width(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).max_advance_width
        }
    }

    #[inline(always)]
    pub fn max_advance_height(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).max_advance_height
        }
    }

    #[inline(always)]
    pub fn underline_position(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).underline_position
        }
    }

    #[inline(always)]
    pub fn underline_thickness(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).underline_thickness
        }
    }

    pub fn family_name(&self) -> Option<String> {

        let family_name_ptr = unsafe { (*self.raw).family_name };

        if family_name_ptr.is_null() {
            None
        } else {
            let family_name = unsafe { c_str_to_bytes(&family_name_ptr) };
            match str::from_utf8(family_name) {
                Ok(string)  => Some(string.to_owned()),
                _           => None
            }
        }
    }

    pub fn style_name(&self) -> Option<String> {

        let style_name_ptr = unsafe { (*self.raw).style_name };

        if style_name_ptr.is_null() {
            None
        } else {
            let style_name = unsafe { c_str_to_bytes(&style_name_ptr) };
            match str::from_utf8(style_name) {
                Ok(string)  => Some(string.to_owned()),
                _           => None
            }
        }
    }
}

impl Drop for Face {
    fn drop(&mut self) {
        unsafe {
            match ffi::FT_Done_Face(self.raw) {
                ffi::FT_Err_Ok => {
                    if ffi::FT_Done_Library(self.library_raw) != ffi::FT_Err_Ok {
                        panic!("Failed to deref library");
                    }
                },
                _ => panic!("Failed to drop face"),
            }
        }
    }
}
