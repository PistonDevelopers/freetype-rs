use std::ffi::{ CString, OsStr };
use std::ptr::null_mut;
use libc::{ self, c_void, c_long, size_t };
use num::FromPrimitive;
use { ffi, Face, FtResult };

extern "C" fn alloc_library(_memory: ffi::FT_Memory, size: c_long) -> *mut c_void {
    unsafe {
        libc::malloc(size as size_t)
    }
}

extern "C" fn free_library(_memory: ffi::FT_Memory, block: *mut c_void) {
    unsafe {
        libc::free(block)
    }
}

extern "C" fn realloc_library(_memory: ffi::FT_Memory,
                              _cur_size: c_long,
                              new_size: c_long,
                              block: *mut c_void) -> *mut c_void {
    unsafe {
        libc::realloc(block, new_size as size_t)
    }
}

static MEMORY: ffi::FT_MemoryRec = ffi::FT_MemoryRec {
    user: 0 as *mut c_void,
    alloc: alloc_library,
    free: free_library,
    realloc: realloc_library,
};


pub struct Library {
    raw: ffi::FT_Library,
}

impl Library {
    pub fn init() -> FtResult<Library> {
        unsafe {
            let mut raw = null_mut();

            let err = ffi::FT_New_Library(&MEMORY, &mut raw);
            if err == ffi::FT_Err_Ok {
                ffi::FT_Add_Default_Modules(raw);
                Ok(Library {
                    raw: raw,
                })
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn new_face<P: AsRef<OsStr>>(&self, filepath: P, face_index: isize) -> FtResult<Face> {
        unsafe {
            let mut face = null_mut();

            let path_str = CString::new(filepath.as_ref().to_str().unwrap()).unwrap();

            let err = ffi::FT_New_Face(self.raw, path_str.as_ptr(), face_index as ffi::FT_Long, &mut face);
            if err == ffi::FT_Err_Ok {
                Ok(Face::from_raw(self.raw, face))
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn new_memory_face(&self, buffer: &[u8], face_index: isize) -> FtResult<Face> {
        unsafe {
            let mut face = null_mut();
            let err = ffi::FT_New_Memory_Face(self.raw, buffer.as_ptr(),
                buffer.len() as ffi::FT_Long, face_index as ffi::FT_Long, &mut face);
            if err == ffi::FT_Err_Ok {
                Ok(Face::from_raw(self.raw, face))
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
        unsafe {
            let err = ffi::FT_Done_Library(self.raw);
            if err != ffi::FT_Err_Ok {
                panic!("Failed to drop freetype library");
            }
        }
    }
}
