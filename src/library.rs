
use std;
use std::num::FromPrimitive;
use ffi::*;
use {
    Face,
    FtResult,
};

pub struct Library {
    raw: FT_Library,
}

impl Library {
    pub fn init() -> FtResult<Library> {
        unsafe {
            let library = std::ptr::null();
            let err = FT_Init_FreeType(&library);
            if err == 0 {
                Ok(Library {
                    raw: library,
                })
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn new_face(&self, filepathname: &str, face_index: FT_Long) -> FtResult<Face> {
        unsafe {
            let face: FT_Face = std::ptr::null();
            let err = FT_New_Face(self.raw, filepathname.as_slice().as_ptr(), face_index, &face);
            if err == FT_Err_Ok {
                Ok(Face::from_raw(face))
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn new_memory_face(&self, buffer: &[u8], face_index: FT_Long) -> FtResult<Face> {
        unsafe {
            let face: FT_Face = std::ptr::null();
            let err = FT_New_Memory_Face(self.raw, buffer.as_ptr(), buffer.len() as i64, face_index, &face);
            if err == FT_Err_Ok {
                Ok(Face::from_raw(face))
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn raw(&self) -> FT_Library {
        self.raw
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        /*
        unsafe {
            let err = FT_Done_FreeType(self.raw);
            if err != 0 {
                std::io::println(format!("Failed to drop Library. Error Code: {}", err).as_slice());
            }
        }
        */
    }
}

