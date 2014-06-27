
use std;
use ffi::*;

pub struct Bitmap {
    raw: *FT_Bitmap,
}

impl Bitmap {
    pub fn from_raw(raw: *FT_Bitmap) -> Bitmap {
        Bitmap {
            raw: raw,
        }
    }

    #[inline(always)]
    pub fn buffer<'a>(&'a self) -> &'a [u8] {
        unsafe {
            std::slice::raw::buf_as_slice((*self.raw).buffer, (self.width() * self.rows()) as uint, |buf: &[u8]| std::mem::transmute(buf))
        }
    }

    #[inline(always)]
    pub fn width(&self) -> i32 {
        unsafe {
            (*self.raw).width
        }
    }

    #[inline(always)]
    pub fn rows(&self) -> i32 {
        unsafe {
            (*self.raw).rows
        }
    }

    #[inline(always)]
    pub fn raw<'a>(&'a self) -> &'a FT_Bitmap {
        unsafe {
            &*self.raw
        }
    }
}

