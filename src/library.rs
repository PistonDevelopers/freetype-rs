
use std;
use std::num::FromPrimitive;
use ffi;
use {
    Face,
    FtResult,
};

pub struct Library {
    raw: ffi::FT_Library,
}

impl Library {
    pub fn init() -> FtResult<Library> {
        unsafe {
            let mut library = std::ptr::mut_null();
            let err = ffi::FT_Init_FreeType(&mut library);
            if err == ffi::FT_Err_Ok {
                Ok(Library {
                    raw: library,
                })
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn new_face(&self, filepathname: &str, face_index: ffi::FT_Long) -> FtResult<Face> {
        unsafe {
            let mut face = std::ptr::mut_null();

            let path_str = filepathname.to_c_str();

            let err = ffi::FT_New_Face(self.raw, path_str.as_ptr(), face_index, &mut face);
            if err == ffi::FT_Err_Ok {
                Ok(Face::from_raw(face))
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn new_memory_face(&self, buffer: &[u8], face_index: ffi::FT_Long) -> FtResult<Face> {
        unsafe {
            let mut face = std::ptr::mut_null();
            let err = ffi::FT_New_Memory_Face(self.raw, buffer.as_ptr(), buffer.len() as ffi::FT_Long, face_index, &mut face);
            if err == ffi::FT_Err_Ok {
                Ok(Face::from_raw(face))
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn raw(&self) -> ffi::FT_Library {
        self.raw
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        /*
        unsafe {
            let err = ffi::FT_Done_FreeType(self.raw);
            if err != 0 {
                std::io::println(format!("Failed to drop Library. Error Code: {}", err).as_slice());
            }
        }
        */
    }
}

