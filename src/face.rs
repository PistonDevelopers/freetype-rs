use std::ffi::{CStr, CString};
use std::fmt;
use std::num::NonZeroU32;
use std::rc::Rc;

use crate::charmap::CharMap;
use crate::{ffi, FtResult, GlyphSlot, Matrix, Vector};

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum KerningMode {
    KerningDefault = ffi::FT_KERNING_DEFAULT,
    KerningUnfitted = ffi::FT_KERNING_UNFITTED,
    KerningUnscaled = ffi::FT_KERNING_UNSCALED,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct LoadFlag: i32 {
        const DEFAULT                    = crate::ffi::FT_LOAD_DEFAULT;
        const NO_SCALE                   = crate::ffi::FT_LOAD_NO_SCALE;
        const NO_HINTING                 = crate::ffi::FT_LOAD_NO_HINTING;
        const RENDER                     = crate::ffi::FT_LOAD_RENDER;
        const NO_BITMAP                  = crate::ffi::FT_LOAD_NO_BITMAP;
        const VERTICAL_LAYOUT            = crate::ffi::FT_LOAD_VERTICAL_LAYOUT;
        const FORCE_AUTOHINT             = crate::ffi::FT_LOAD_FORCE_AUTOHINT;
        const CROP_BITMAP                = crate::ffi::FT_LOAD_CROP_BITMAP;
        const PEDANTIC                   = crate::ffi::FT_LOAD_PEDANTIC;
        const IGNORE_GLOBAL_ADVANCE_WITH = crate::ffi::FT_LOAD_IGNORE_GLOBAL_ADVANCE_WIDTH;
        const NO_RECURSE                 = crate::ffi::FT_LOAD_NO_RECURSE;
        const IGNORE_TRANSFORM           = crate::ffi::FT_LOAD_IGNORE_TRANSFORM;
        const MONOCHROME                 = crate::ffi::FT_LOAD_MONOCHROME;
        const LINEAR_DESIGN              = crate::ffi::FT_LOAD_LINEAR_DESIGN;
        const NO_AUTOHINT                = crate::ffi::FT_LOAD_NO_AUTOHINT;
        const TARGET_NORMAL              = crate::ffi::FT_LOAD_TARGET_NORMAL;
        const TARGET_LIGHT               = crate::ffi::FT_LOAD_TARGET_LIGHT;
        const TARGET_MONO                = crate::ffi::FT_LOAD_TARGET_MONO;
        const TARGET_LCD                 = crate::ffi::FT_LOAD_TARGET_LCD;
        const TARGET_LCD_V               = crate::ffi::FT_LOAD_TARGET_LCD_V;
        const COLOR                      = crate::ffi::FT_LOAD_COLOR;
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StyleFlag: ffi::FT_Long {
        const BOLD   = crate::ffi::FT_STYLE_FLAG_BOLD;
        const ITALIC = crate::ffi::FT_STYLE_FLAG_ITALIC;
    }
}

pub struct CharIterator<'a, BYTES> {
    started: bool,
    face: &'a Face<BYTES>,
    gindex: ffi::FT_UInt,
    charcode: ffi::FT_ULong,
}

impl<'a, BYTES> CharIterator<'a, BYTES> {
    fn new(face: &'a Face<BYTES>) -> Self {
        CharIterator {
            started: false,
            face,
            gindex: 0,
            charcode: 0,
        }
    }
}

impl<'a, BYTES> Iterator for CharIterator<'a, BYTES> {
    type Item = (usize, NonZeroU32);

    fn next(&mut self) -> Option<Self::Item> {
        // Implementing per https://freetype.org/freetype2/docs/reference/ft2-character_mapping.html#ft_get_first_char
        //  FT_UInt   gindex;
        //  FT_ULong charcode = FT_Get_First_Char( face, &gindex );
        //  while ( gindex != 0 ) {
        //    ... do something with (charcode,gindex) pair ...
        //    charcode = FT_Get_Next_Char( face, charcode, &gindex );
        //  }

        if self.started {
            self.charcode =
                unsafe { ffi::FT_Get_Next_Char(self.face.raw, self.charcode, &mut self.gindex) };
        } else {
            self.started = true;
            self.charcode = unsafe { ffi::FT_Get_First_Char(self.face.raw, &mut self.gindex) };
        }

        if self.gindex == 0 {
            None
        } else {
            NonZeroU32::new(self.gindex).map(|gindex| (self.charcode as usize, gindex))
        }
    }

    // TODO: implement size_hint
}

#[derive(Eq, PartialEq, Hash)]
pub struct Face<BYTES = Rc<Vec<u8>>> {
    library_raw: ffi::FT_Library,
    raw: ffi::FT_Face,
    glyph: GlyphSlot,
    bytes: Option<BYTES>,
}

impl<BYTES: Clone> Clone for Face<BYTES> {
    fn clone(&self) -> Self {
        let err = unsafe { ffi::FT_Reference_Library(self.library_raw) };
        if err != ffi::FT_Err_Ok {
            panic!("Failed to reference library");
        }
        let err = unsafe { ffi::FT_Reference_Face(self.raw) };
        if err != ffi::FT_Err_Ok {
            panic!("Failed to reference face");
        }
        Face {
            library_raw: self.library_raw,
            raw: self.raw,
            glyph: self.glyph,
            bytes: self.bytes.clone(),
        }
    }
}

impl<BYTES> Face<BYTES> {
    pub unsafe fn from_raw(
        library_raw: ffi::FT_Library,
        raw: ffi::FT_Face,
        bytes: Option<BYTES>,
    ) -> Self {
        ffi::FT_Reference_Library(library_raw);
        Face {
            library_raw,
            raw,
            glyph: GlyphSlot::from_raw(library_raw, (*raw).glyph),
            bytes,
        }
    }

    pub fn attach_file(&self, filepathname: &str) -> FtResult<()> {
        let err = unsafe { ffi::FT_Attach_File(self.raw, filepathname.as_ptr() as *const _) };
        if err == ffi::FT_Err_Ok {
            Ok(())
        } else {
            Err(err.into())
        }
    }

    pub fn reference(&self) -> FtResult<()> {
        let err = unsafe { ffi::FT_Reference_Face(self.raw) };
        if err == ffi::FT_Err_Ok {
            Ok(())
        } else {
            Err(err.into())
        }
    }

    pub fn set_char_size(
        &self,
        char_width: isize,
        char_height: isize,
        horz_resolution: u32,
        vert_resolution: u32,
    ) -> FtResult<()> {
        let err = unsafe {
            ffi::FT_Set_Char_Size(
                self.raw,
                char_width as ffi::FT_F26Dot6,
                char_height as ffi::FT_F26Dot6,
                horz_resolution,
                vert_resolution,
            )
        };
        if err == ffi::FT_Err_Ok {
            Ok(())
        } else {
            Err(err.into())
        }
    }

    pub fn select_size(&self, strike_index: i32) -> FtResult<()> {
        let err = unsafe { ffi::FT_Select_Size(self.raw, strike_index) };
        if err == ffi::FT_Err_Ok {
            Ok(())
        } else {
            Err(err.into())
        }
    }

    pub fn set_pixel_sizes(&self, pixel_width: u32, pixel_height: u32) -> FtResult<()> {
        let err = unsafe { ffi::FT_Set_Pixel_Sizes(self.raw, pixel_width, pixel_height) };
        if err == ffi::FT_Err_Ok {
            Ok(())
        } else {
            Err(err.into())
        }
    }

    pub fn load_glyph(&self, glyph_index: u32, load_flags: LoadFlag) -> FtResult<()> {
        let err = unsafe { ffi::FT_Load_Glyph(self.raw, glyph_index, load_flags.bits()) };
        if err == ffi::FT_Err_Ok {
            Ok(())
        } else {
            Err(err.into())
        }
    }

    pub fn load_char(&self, char_code: usize, load_flags: LoadFlag) -> FtResult<()> {
        let err =
            unsafe { ffi::FT_Load_Char(self.raw, char_code as ffi::FT_ULong, load_flags.bits()) };
        if err == ffi::FT_Err_Ok {
            Ok(())
        } else {
            Err(err.into())
        }
    }

    pub fn set_transform(&self, matrix: &mut Matrix, delta: &mut Vector) {
        unsafe {
            ffi::FT_Set_Transform(self.raw, matrix, delta);
        }
    }

    pub fn get_char_index(&self, charcode: usize) -> Option<u32> {
        let res = unsafe { ffi::FT_Get_Char_Index(self.raw, charcode as ffi::FT_ULong) };
        if res == 0 {
            None
        } else {
            Some(res)
        }
    }

    pub fn get_name_index(&self, glyph_name: &str) -> Option<u32> {
        if !unsafe { ffi::FT_HAS_GLYPH_NAMES(self.raw) } {
            return None;
        }

        match CString::new(glyph_name) {
            Ok(name) => {
                Some(unsafe { ffi::FT_Get_Name_Index(self.raw, name.as_ptr() as *const _) })
            }
            Err(_) => None,
        }
    }

    pub fn chars(&self) -> CharIterator<'_, BYTES> {
        CharIterator::new(self)
    }

    pub fn get_kerning(
        &self,
        left_char_index: u32,
        right_char_index: u32,
        kern_mode: KerningMode,
    ) -> FtResult<Vector> {
        let mut vec = Vector { x: 0, y: 0 };

        let err = unsafe {
            ffi::FT_Get_Kerning(
                self.raw,
                left_char_index,
                right_char_index,
                kern_mode as u32,
                &mut vec,
            )
        };
        if err == ffi::FT_Err_Ok {
            Ok(vec)
        } else {
            Err(err.into())
        }
    }

    pub fn get_charmap(&self, charmap_index: isize) -> CharMap {
        let charmap = unsafe { *self.raw().charmaps.offset(charmap_index) };
        CharMap::new(charmap)
    }

    pub fn set_charmap(&self, charmap: &CharMap) -> FtResult<()> {
        let err = unsafe { ffi::FT_Set_Charmap(self.raw, charmap.raw()) };
        if err == ffi::FT_Err_Ok {
            Ok(())
        } else {
            Err(err.into())
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
        unsafe { ffi::FT_HAS_HORIZONTAL(self.raw) }
    }

    #[inline(always)]
    pub fn has_vertical(&self) -> bool {
        unsafe { ffi::FT_HAS_VERTICAL(self.raw) }
    }

    #[inline(always)]
    pub fn has_kerning(&self) -> bool {
        unsafe { ffi::FT_HAS_KERNING(self.raw) }
    }

    #[inline(always)]
    pub fn is_scalable(&self) -> bool {
        unsafe { ffi::FT_IS_SCALABLE(self.raw) }
    }

    #[inline(always)]
    pub fn is_sfnt(&self) -> bool {
        unsafe { ffi::FT_IS_SFNT(self.raw) }
    }

    #[inline(always)]
    pub fn is_fixed_width(&self) -> bool {
        unsafe { ffi::FT_IS_FIXED_WIDTH(self.raw) }
    }

    #[inline(always)]
    pub fn has_fixed_sizes(&self) -> bool {
        unsafe { ffi::FT_HAS_FIXED_SIZES(self.raw) }
    }

    #[inline(always)]
    pub fn has_glyph_names(&self) -> bool {
        unsafe { ffi::FT_HAS_GLYPH_NAMES(self.raw) }
    }

    #[inline(always)]
    pub fn is_cid_keyed(&self) -> bool {
        unsafe { ffi::FT_IS_CID_KEYED(self.raw) }
    }

    #[inline(always)]
    pub fn is_tricky(&self) -> bool {
        unsafe { ffi::FT_IS_TRICKY(self.raw) }
    }

    #[inline(always)]
    pub fn has_color(&self) -> bool {
        unsafe { ffi::FT_HAS_COLOR(self.raw) }
    }

    #[inline(always)]
    pub fn raw(&self) -> &ffi::FT_FaceRec {
        unsafe { &*self.raw }
    }

    #[inline(always)]
    pub fn raw_mut(&mut self) -> &mut ffi::FT_FaceRec {
        unsafe { &mut *self.raw }
    }

    #[inline(always)]
    pub fn ascender(&self) -> ffi::FT_Short {
        unsafe { (*self.raw).ascender }
    }

    #[inline(always)]
    pub fn num_charmaps(&self) -> i32 {
        unsafe { (*self.raw).num_charmaps }
    }

    #[inline(always)]
    pub fn descender(&self) -> ffi::FT_Short {
        unsafe { (*self.raw).descender }
    }

    #[inline(always)]
    pub fn em_size(&self) -> ffi::FT_Short {
        unsafe { (*self.raw).units_per_EM as i16 }
    }

    #[inline(always)]
    pub fn height(&self) -> ffi::FT_Short {
        unsafe { (*self.raw).height }
    }

    #[inline(always)]
    pub fn max_advance_width(&self) -> ffi::FT_Short {
        unsafe { (*self.raw).max_advance_width }
    }

    #[inline(always)]
    pub fn max_advance_height(&self) -> ffi::FT_Short {
        unsafe { (*self.raw).max_advance_height }
    }

    #[inline(always)]
    pub fn underline_position(&self) -> ffi::FT_Short {
        unsafe { (*self.raw).underline_position }
    }

    #[inline(always)]
    pub fn underline_thickness(&self) -> ffi::FT_Short {
        unsafe { (*self.raw).underline_thickness }
    }

    #[inline(always)]
    pub fn num_faces(&self) -> ffi::FT_Short {
        unsafe { (*self.raw).num_faces as i16 }
    }

    #[inline(always)]
    pub fn num_glyphs(&self) -> ffi::FT_Long {
        unsafe { (*self.raw).num_glyphs }
    }

    pub fn family_name(&self) -> Option<String> {
        let family_name = unsafe { (*self.raw).family_name };

        if family_name.is_null() {
            None
        } else {
            let family_name =
                unsafe { CStr::from_ptr(family_name as *const _).to_bytes().to_vec() };
            String::from_utf8(family_name).ok()
        }
    }

    pub fn style_name(&self) -> Option<String> {
        let style_name = unsafe { (*self.raw).style_name };

        if style_name.is_null() {
            None
        } else {
            let style_name = unsafe { CStr::from_ptr(style_name as *const _).to_bytes().to_vec() };
            String::from_utf8(style_name).ok()
        }
    }

    pub fn style_flags(&self) -> StyleFlag {
        let style_flags = unsafe { (*self.raw).style_flags };
        StyleFlag::from_bits_truncate(style_flags)
    }

    pub fn size_metrics(&self) -> Option<ffi::FT_Size_Metrics> {
        if self.raw.is_null() {
            None
        } else {
            let size = unsafe { (*self.raw).size };
            if size.is_null() {
                None
            } else {
                Some(unsafe { (*size).metrics })
            }
        }
    }

    pub fn postscript_name(&self) -> Option<String> {
        let face_name = unsafe { ffi::FT_Get_Postscript_Name(self.raw) };
        if face_name.is_null() {
            None
        } else {
            let face_name = unsafe { CStr::from_ptr(face_name as *const _).to_bytes().to_vec() };
            String::from_utf8(face_name).ok()
        }
    }
}

impl<BYTES> fmt::Debug for Face<BYTES> {
    fn fmt(&self, form: &mut fmt::Formatter) -> fmt::Result {
        let name = self.style_name().unwrap_or("[unknown name]".to_owned());
        form.write_str("Font Face: ")?;
        form.write_str(&name[..])
    }
}

impl<BYTES> Drop for Face<BYTES> {
    fn drop(&mut self) {
        let err = unsafe { ffi::FT_Done_Face(self.raw) };
        if err != ffi::FT_Err_Ok {
            panic!("Failed to drop face");
        }
        let err = unsafe { ffi::FT_Done_Library(self.library_raw) };
        if err != ffi::FT_Err_Ok {
            panic!("Failed to drop library")
        }
        self.bytes = None;
    }
}
