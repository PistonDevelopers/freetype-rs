use { ffi, Bitmap };

#[derive(Copy, Clone)]
pub struct BitmapGlyph {
    raw: ffi::FT_BitmapGlyph
}

impl BitmapGlyph {
    pub fn from_raw(raw: ffi::FT_BitmapGlyph) -> Self {
        BitmapGlyph {
            raw: raw
        }
    }

    #[inline(always)]
    pub fn left(&self) -> i32 {
        unsafe {
            (*self.raw).left
        }
    }

    #[inline(always)]
    pub fn top(&self) -> i32 {
        unsafe {
            (*self.raw).top
        }
    }

    #[inline(always)]
    pub fn bitmap(&self) -> Bitmap {
        let bitmap = unsafe { &(*self.raw).bitmap };

        Bitmap::from_raw(bitmap)
    }

    #[inline(always)]
    pub fn raw(&self) -> &ffi::FT_BitmapGlyphRec {
        unsafe {
            &*self.raw
        }
    }
}
