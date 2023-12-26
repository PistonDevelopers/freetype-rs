use freetype_sys::FT_CharMap;

pub struct CharMap {
    raw: FT_CharMap,
}

impl CharMap {
    pub fn new(raw: FT_CharMap) -> Self {
        CharMap { raw }
    }

    pub fn platform_id(&self) -> u16 {
        unsafe { (*self.raw).platform_id }
    }

    pub fn encoding_id(&self) -> u16 {
        unsafe { (*self.raw).encoding_id }
    }

    pub fn encoding(&self) -> u32 {
        unsafe { (*self.raw).encoding }
    }
}
