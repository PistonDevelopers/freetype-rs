use { ffi };

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StrokerLineCap {
    Butt = ffi::FT_STROKER_LINECAP_BUTT,
    Round = ffi::FT_STROKER_LINECAP_ROUND,
    Square = ffi::FT_STROKER_LINECAP_SQUARE,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StrokerLineJoin {
    Round = ffi::FT_STROKER_LINEJOIN_ROUND,
    Bevel = ffi::FT_STROKER_LINEJOIN_BEVEL,
    MiterVariable = ffi::FT_STROKER_LINEJOIN_MITER_VARIABLE,
    MiterFixed = ffi::FT_STROKER_LINEJOIN_MITER_FIXED,
}

pub struct Stroker {
    library_raw: ffi::FT_Library,
    raw: ffi::FT_Stroker,
}

impl Stroker {
    pub unsafe fn from_raw(library_raw: ffi::FT_Library, raw: ffi::FT_Stroker) -> Self {
        ffi::FT_Reference_Library(library_raw);
        Stroker {
            library_raw: library_raw,
            raw: raw,
        }
    }

    pub fn set(&self, radius: ffi::FT_Fixed, line_cap: StrokerLineCap, line_join: StrokerLineJoin, miter_limit: ffi::FT_Fixed) {
        unsafe {
            ffi::FT_Stroker_Set(self.raw, radius, line_cap as u32, line_join as u32, miter_limit);
        }
    }

    pub fn raw(&self) -> &ffi::FT_StrokerRec {
        unsafe {
            &*self.raw
        }
    }

    pub fn raw_mut(&mut self) -> &mut ffi::FT_StrokerRec {
        unsafe {
            &mut *self.raw
        }
    }

    pub(crate) fn raw_stroker(&self) -> ffi::FT_Stroker {
        self.raw
    }
}

impl Drop for Stroker {

    fn drop(&mut self) {
        let err = unsafe {
            ffi::FT_Stroker_Done(self.raw);
            ffi::FT_Done_Library(self.library_raw)
        };
        if err != ffi::FT_Err_Ok {
            panic!("Failed to drop library");
        }
    }
}
