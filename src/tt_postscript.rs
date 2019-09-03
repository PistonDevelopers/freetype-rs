use ffi;
use face::Face;

#[derive(Copy, Clone)]
pub struct TrueTypePostscriptTable {
    raw: ffi::TT_Postscript_Internal
}

impl TrueTypePostscriptTable  {
    pub fn from_face(face: &mut Face) -> Option<Self> {
        unsafe {
            let post = ffi::FT_Get_Sfnt_Table(face.raw_mut() as *mut ffi::FT_FaceRec, ffi::ft_sfnt_post) as ffi::TT_Postscript_Internal;
            if !post.is_null() && (*post).formatType != 0 {
                Some(TrueTypePostscriptTable {
                    raw: post
                })
            } else {
                None
            }
        }
    }

    #[inline(always)]
    pub fn format_type(&self) -> ffi::FT_Fixed {
        unsafe {
            (*self.raw).formatType
        }
    }

    #[inline(always)]
    pub fn italic_angle(&self) -> ffi::FT_Fixed {
        unsafe {
            (*self.raw).italicAngle
        }
    }

    #[inline(always)]
    pub fn underline_position(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).underlinePosition
        }
    }

    #[inline(always)]
    pub fn underline_thickness(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).underlineThickness
        }
    }

    #[inline(always)]
    pub fn is_fixed_pitch(&self) -> ffi::FT_ULong {
        unsafe {
            (*self.raw).isFixedPitch
        }
    }

    #[inline(always)]
    pub fn min_mem_type_42(&self) -> ffi::FT_ULong {
        unsafe {
            (*self.raw).minMemType42
        }
    }

    #[inline(always)]
    pub fn max_mem_type_42(&self) -> ffi::FT_ULong {
        unsafe {
            (*self.raw).maxMemType42
        }
    }

    #[inline(always)]
    pub fn min_mem_type_1(&self) -> ffi::FT_ULong {
        unsafe {
            (*self.raw).minMemType1
        }
    }

    #[inline(always)]
    pub fn max_mem_type_1(&self) -> ffi::FT_ULong {
        unsafe {
            (*self.raw).maxMemType1
        }
    }
}
