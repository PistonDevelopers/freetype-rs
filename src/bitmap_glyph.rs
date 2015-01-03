
use ffi;
use {
    Bitmap,
};

#[derive(Copy)]
pub struct BitmapGlyph {
    raw: ffi::FT_BitmapGlyph,
}

impl BitmapGlyph {
    pub fn from_raw(raw: ffi::FT_BitmapGlyph) -> BitmapGlyph {
        BitmapGlyph {
            raw: raw,
        }
    }

    #[inline(always)]
    pub fn left(&self) -> ffi::FT_Int {
        unsafe {
            (*self.raw).left
        }
    }

    #[inline(always)]
    pub fn top(&self) -> ffi::FT_Int {
        unsafe {
            (*self.raw).top
        }
    }

    #[inline(always)]
    pub fn bitmap(&self) -> Bitmap {
        unsafe {
            Bitmap::from_raw(&(*self.raw).bitmap)
        }
    }

    #[inline(always)]
    pub fn raw(&self) -> &ffi::FT_BitmapGlyphRec {
        unsafe {
            &*self.raw
        }
    }
}
