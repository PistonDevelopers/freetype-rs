
use std;
use std::num::FromPrimitive;
use std::c_str::CString;
use ffi;
use {
    FtResult,
    GlyphSlot,
    Matrix,
    Vector,
};

#[repr(u32)]
pub enum KerningMode {
    KerningDefault = ffi::FT_KERNING_DEFAULT,
    KerningUnfitted = ffi::FT_KERNING_UNFITTED,
    KerningUnscaled = ffi::FT_KERNING_UNSCALED
}

bitflags!(flags LoadFlag: i32 {
    const Default = ffi::FT_LOAD_DEFAULT,
    const NoScale = ffi::FT_LOAD_NO_SCALE,
    const NoHinting = ffi::FT_LOAD_NO_HINTING,
    const Render = ffi::FT_LOAD_RENDER,
    const NoBitmap = ffi::FT_LOAD_NO_BITMAP,
    const VerticalLayout = ffi::FT_LOAD_VERTICAL_LAYOUT,
    const ForceAutohint = ffi::FT_LOAD_FORCE_AUTOHINT,
    const CropBitmap = ffi::FT_LOAD_CROP_BITMAP,
    const Pendantic = ffi::FT_LOAD_PENDANTIC,
    const IgnoreGlobalAdvanceWidth = ffi::FT_LOAD_IGNORE_GLOBAL_ADVANCE_WIDTH,
    const NoRecurse = ffi::FT_LOAD_NO_RECURSE,
    const IgnoreTransform = ffi::FT_LOAD_IGNORE_TRANSFORM,
    const Monochrome = ffi::FT_LOAD_MONOCHROME,
    const LinearDesign = ffi::FT_LOAD_LINEAR_DESIGN,
    const NoAutohint = ffi::FT_LOAD_NO_AUTOHINT,
    const LoadTargetNormal = ffi::FT_LOAD_TARGET_NORMAL,
    const LoadTargetLight = ffi::FT_LOAD_TARGET_LIGHT,
    const LoadTargetMono = ffi::FT_LOAD_TARGET_MONO,
    const LoadTargetLCD = ffi::FT_LOAD_TARGET_LCD,
    const LoadTargetLCD_V = ffi::FT_LOAD_TARGET_LCD_V,
    const Color = ffi::FT_LOAD_COLOR
})

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

    pub fn get_kerning(&self, left_char_index: ffi::FT_UInt, right_char_index: ffi::FT_UInt, kern_mode: KerningMode)
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
            let family_name = unsafe { CString::new(family_name_ptr, false) };
            Some(family_name.as_str().unwrap().into_string())
        }
    }

    pub fn style_name(&self) -> Option<String> {

        let style_name_ptr = unsafe { (*self.raw).style_name };

        if style_name_ptr.is_null() {
            None
        } else {
            let style_name = unsafe { CString::new(style_name_ptr, false) };
            Some(style_name.as_str().unwrap().into_string())
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
            ffi::FT_Done_Library(self.library_raw);
        }
    }
}

