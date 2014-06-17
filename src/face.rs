
use std;
use ffi::*;
use Library;

pub struct Face {
    raw: FT_Face,
}

impl Face {
    pub fn new(library: &Library, filepathname: &str, face_index: i64) -> Result<Face, String> {
        unsafe {
            let face: FT_Face = std::ptr::null();
            let err = FT_New_Face(library.raw(), filepathname.as_slice().as_ptr(), face_index, &face);
            if err == 0 {
                Ok(Face {
                    raw: face,
                })
            } else {
                Err(format!("Failed to create face for '{}'. Error Code: {}", filepathname, err))
            }
        }
    }

    pub fn attach_file(&self, filepathname: &str) -> Result<(), String> {
        unsafe {
            let err = FT_Attach_File(self.raw(), filepathname.as_slice().as_ptr() as *i8);
            if err == 0 {
                Ok(())
            } else {
                Err(format!("Failed to attach file '{}'. Error Code: {}", filepathname, err))
            }
        }
    }

    pub fn reference(&self) -> Result<(), String> {
        unsafe {
            let err = FT_Reference_Face(self.raw());
            if err == 0 {
                Ok(())
            } else {
                Err(format!("Failed to reference face. Error Code: {}", err))
            }
        }
    }

    pub fn set_char_size(&self, char_width: i64, char_height: i64, horz_resolution: u32, vert_resolution: u32) -> Result<(), String> {
        unsafe {
            let err = FT_Set_Char_Size(self.raw(), char_width, char_height, horz_resolution, vert_resolution);
            if err == 0 {
                Ok(())
            } else {
                Err(format!("Failed to set character size. Error Code: {}", err))
            }
        }
    }

    pub fn load_glyph(&self, glyph_index: u32, load_flags: LoadFlag) -> Result<(), String> {
        unsafe {
            let err = FT_Load_Glyph(self.raw(), glyph_index, load_flags.bits);
            if err == 0 {
                Ok(())
            } else {
                Err(format!("Failed to load glyph of index {}. Error Code: {}", glyph_index, err))
            }
        }
    }

    pub fn load_char(&self, char_code: u64, load_flags: LoadFlag) -> Result<(), String> {
        unsafe {
            let err = FT_Load_Char(self.raw(), char_code, load_flags.bits);
            if err == 0 {
                Ok(())
            } else {
                Err(format!("Failed to load character {}. Error Code: {}", char_code, err))
            }
        }
    }

    pub fn glyph<'a>(&'a self) -> &'a FT_GlyphSlotRec {
        unsafe {
            &*(*self.raw).glyph
        }
    }

    pub fn raw(&self) -> FT_Face {
        self.raw
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

