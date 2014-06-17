
use std;
use ffi::*;

pub struct Library {
    raw: FT_Library,
}

impl Library {
    pub fn init() -> Result<Library, String> {
        unsafe {
            let library = std::ptr::null();
            let err = FT_Init_FreeType(&library);
            if err == 0 {
                Ok(Library {
                    raw: library,
                })
            } else {
                Err(format!("Failed to initialize FreeType. Error Code: {}", err))
            }
        }
    }

    pub fn raw(&self) -> FT_Library {
        self.raw
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe {
            let err = FT_Done_FreeType(self.raw());
            if err != 0 {
                std::io::println(format!("Failed to drop Library. Error Code: {}", err).as_slice());
            }
        }
    }
}

