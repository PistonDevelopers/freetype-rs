use ffi;
use face::Face;

#[derive(Copy, Clone)]
pub struct TrueTypeOS2Table {
    raw: ffi::TT_OS2_Internal
}

impl TrueTypeOS2Table  {
    pub fn from_face(face: &mut Face) -> Option<Self> {
        unsafe {
            let os2 = ffi::FT_Get_Sfnt_Table(face.raw_mut() as *mut ffi::FT_FaceRec, ffi::ft_sfnt_os2) as ffi::TT_OS2_Internal;
            if !os2.is_null() && (*os2).version != 0xffff {
                Some(TrueTypeOS2Table {
                    raw: os2
                })
            } else {
                None
            }
        }
    }

    #[inline(always)]
    pub fn version(&self) -> ffi::FT_UShort {
        unsafe {
            (*self.raw).version
        }
    }

    #[inline(always)]
    pub fn avg_char_width(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).xAvgCharWidth
        }
    }

    #[inline(always)]
    pub fn us_weight_class(&self) -> ffi::FT_UShort {
        unsafe {
            (*self.raw).usWeightClass
        }
    }

    #[inline(always)]
    pub fn us_width_class(&self) -> ffi::FT_UShort {
        unsafe {
            (*self.raw).usWidthClass
        }
    }

    #[inline(always)]
    pub fn fs_type(&self) -> ffi::FT_UShort {
        unsafe {
            (*self.raw).fsType
        }
    }


    #[inline(always)]
    pub fn y_subscript_x_size(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).ySubscriptXSize
        }
    }

    #[inline(always)]
    pub fn y_subscript_y_size(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).ySubscriptYSize
        }
    }

    #[inline(always)]
    pub fn y_subscript_x_offset(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).ySubscriptXOffset
        }
    }


    #[inline(always)]
    pub fn y_subscript_y_offset(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).ySubscriptYOffset
        }
    }

    #[inline(always)]
    pub fn y_superscript_x_size(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).ySuperscriptXSize
        }
    }

    #[inline(always)]
    pub fn y_superscript_y_size(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).ySuperscriptYSize
        }
    }

    #[inline(always)]
    pub fn y_superscript_x_offset(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).ySuperscriptXOffset
        }
    }

    #[inline(always)]
    pub fn y_superscript_y_offset(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).ySuperscriptYOffset
        }
    }

    #[inline(always)]
    pub fn y_strikeout_size(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).yStrikeoutSize
        }
    }

    #[inline(always)]
    pub fn y_strikeout_position(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).yStrikeoutPosition
        }
    }

    #[inline(always)]
    pub fn s_family_class(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).sFamilyClass
        }
    }

    #[inline(always)]
    pub fn fs_selection(&self) -> ffi::FT_UShort {
        unsafe {
            (*self.raw).fsSelection
        }
    }

    #[inline(always)]
    pub fn s_typo_ascender(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).sTypoAscender
        }
    }

    #[inline(always)]
    pub fn s_typo_descender(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).sTypoDescender
        }
    }

    #[inline(always)]
    pub fn s_typo_line_gap(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).sTypoLineGap
        }
    }

    #[inline(always)]
    pub fn x_height(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).sxHeight
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use library::Library;

    use super::*;

    /// Sanity-check reading basic line metrics from the OS/2 table.
    #[test]
    fn line_metrics() {
        let mut fira_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        fira_path.push("examples/assets/FiraSans-Regular.ttf");

        let library = Library::init().unwrap();
        let mut face = library.new_face(fira_path, 0).unwrap();
        let os2 = TrueTypeOS2Table::from_face(&mut face).unwrap();

        assert_eq!(os2.s_typo_ascender(), 785);
        assert_eq!(os2.s_typo_descender(), -215);
        assert_eq!(os2.s_typo_line_gap(), 400);
    }
}
