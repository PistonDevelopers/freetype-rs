use std::slice;
use { ffi, FtResult, Error };

/// An enumeration type used to describe the format of pixels in a given bitmap. Note that
/// additional formats may be added in the future.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PixelMode {
    /// This value is reserved.
    None,

    /// A monochrome bitmap, using 1 bit per pixel. Note that pixels are
    /// stored in most-significant order (MSB), which means that the left-most
    /// pixel in a byte has value 128.
    Mono,

    /// An 8-bit bitmap, generally used to represent anti-aliased glyph images.
    /// Each pixel is stored in one byte. Note that the number of `gray`
    /// levels is stored in the `num_grays` field of the FT_Bitmap structure
    /// (it generally is 256).
    Gray,

    /// A 2-bit per pixel bitmap, used to represent embedded anti-aliased
    /// bitmaps in font files according to the OpenType specification. We
    /// haven't found a single font using this format, however.
    Gray2,

    /// A 4-bit per pixel bitmap, representing embedded anti-aliased bitmaps in
    /// font files according to the OpenType specification. We haven't found a
    /// single font using this format, however.
    Gray4,

    /// An 8-bit bitmap, representing RGB or BGR decimated glyph images used
    /// for display on LCD displays; the bitmap is three times wider than the
    /// original glyph image. See also FT_RENDER_MODE_LCD.
    Lcd,

    /// An 8-bit bitmap, representing RGB or BGR decimated glyph images used for
    /// display on rotated LCD displays; the bitmap is three times taller than
    /// the original glyph image. See also FT_RENDER_MODE_LCD_V.
    LcdV,

    /// An image with four 8-bit channels per pixel, representing a color image
    /// (such as emoticons) with alpha channel. For each pixel, the format is
    /// BGRA, which means, the blue channel comes first in memory. The color
    /// channels are pre-multiplied and in the sRGB colorspace. For example,
    /// full red at half-translucent opacity will be represented as
    /// `00,00,80,80`, not `00,00,FF,80`. See also FT_LOAD_COLOR.
    Bgra
}

#[allow(missing_copy_implementations)]
pub struct Bitmap {
    raw: *const ffi::FT_Bitmap
}

impl Bitmap {
    pub unsafe fn from_raw(raw: *const ffi::FT_Bitmap) -> Self {
        Bitmap {
            raw: raw
        }
    }

    /// A typeless pointer to the bitmap buffer. This value should be aligned
    /// on 32-bit boundaries in most cases.
    pub fn buffer(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                (*self.raw).buffer,
                (self.pitch().abs() * self.rows()) as usize
            )
        }
    }

    /// The number of pixels in bitmap row.
    pub fn width(&self) -> i32 {
        unsafe {
            (*self.raw).width
        }
    }

    /// The number of bitmap rows.
    pub fn rows(&self) -> i32 {
        unsafe {
            (*self.raw).rows
        }
    }

    pub fn raw(&self) -> &ffi::FT_Bitmap {
        unsafe {
            &*self.raw
        }
    }

    /// The pixel mode, i.e., how pixel bits are stored. See `PixelMode` for
    /// possible values.
    pub fn pixel_mode(&self) -> FtResult<PixelMode> {
        let pixel_mode = unsafe { (*self.raw).pixel_mode } as u32;

        Ok(match pixel_mode {
            ffi::FT_PIXEL_MODE_NONE  => PixelMode::None,
            ffi::FT_PIXEL_MODE_MONO  => PixelMode::Mono,
            ffi::FT_PIXEL_MODE_GRAY  => PixelMode::Gray,
            ffi::FT_PIXEL_MODE_GRAY2 => PixelMode::Gray2,
            ffi::FT_PIXEL_MODE_GRAY4 => PixelMode::Gray4,
            ffi::FT_PIXEL_MODE_LCD   => PixelMode::Lcd,
            ffi::FT_PIXEL_MODE_LCD_V => PixelMode::LcdV,
            ffi::FT_PIXEL_MODE_BGRA  => PixelMode::Bgra,
            _ => return Err(Error::UnexpectedPixelMode)
        })
    }

    /// The pitch's absolute value is the number of bytes taken by one bitmap row, including
    /// padding. However, the pitch is positive when the bitmap has a ‘down’ flow, and negative
    /// when it has an ‘up’ flow. In all cases, the pitch is an offset to add to a bitmap pointer
    /// in order to go down one row.
    ///
    /// Note that ‘padding’ means the alignment of a bitmap to a byte border, and FreeType
    /// functions normally align to the smallest possible integer value.
    /// For the B/W rasterizer, ‘pitch’ is always an even number.
    ///
    /// To change the pitch of a bitmap (say, to make it a multiple of 4), use FT_Bitmap_Convert.
    /// Alternatively, you might use callback functions to directly render to the application's
    /// surface; see the file ‘example2.cpp’ in the tutorial for a demonstration.
    pub fn pitch(&self) -> i32 {
        unsafe {
            (*self.raw).pitch
        }
    }
}
