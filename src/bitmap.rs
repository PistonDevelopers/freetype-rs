
use std;
use ffi;

pub struct Bitmap {
    raw: *const ffi::FT_Bitmap,
}

impl Bitmap {
    pub fn from_raw(raw: *const ffi::FT_Bitmap) -> Bitmap {
        Bitmap {
            raw: raw,
        }
    }

    #[inline(always)]
    pub fn buffer(&self) -> &[u8] {
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
    pub fn raw(&self) -> &ffi::FT_Bitmap {
        unsafe {
            &*self.raw
        }
    }
}

