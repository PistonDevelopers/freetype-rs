use ffi;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum RenderMode {
    Normal = ffi::FT_RENDER_MODE_NORMAL,
    Light  = ffi::FT_RENDER_MODE_LIGHT,
    Mono   = ffi::FT_RENDER_MODE_MONO,
    Lcd    = ffi::FT_RENDER_MODE_LCD,
    LcdV   = ffi::FT_RENDER_MODE_LCD_V,
    Max    = ffi::FT_RENDER_MODE_MAX
}
