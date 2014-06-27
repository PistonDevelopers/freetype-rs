
use ffi::*;
use {
    Bitmap,
};

pub struct BitmapGlyph {
    raw: FT_BitmapGlyph,
}

impl BitmapGlyph {
    pub fn from_raw(raw: FT_BitmapGlyph) -> BitmapGlyph {
        BitmapGlyph {
            raw: raw,
        }
    }

    #[inline(always)]
    pub fn left(&self) -> FT_Int {
        unsafe {
            (*self.raw).left
        }
    }

    #[inline(always)]
    pub fn top(&self) -> FT_Int {
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
    pub fn raw<'a>(&'a self) -> &'a FT_BitmapGlyphRec {
        unsafe {
            &*self.raw
        }
    }
}
