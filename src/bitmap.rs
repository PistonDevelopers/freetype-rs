
use std;
use ffi;

#[allow(missing_copy_implementations)]
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
            std::slice::from_raw_buf(
                &(*self.raw).buffer, 
                (self.width() * self.rows()) as uint, 
            )
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

