
use libc;
use libc::{
    c_void,
    c_long,
    size_t,
};
use std;
use std::num::FromPrimitive;
use ffi;
use {
    Face,
    FtResult,
};

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
    user: 0 as *const c_void,
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
            let mut raw = std::ptr::null_mut();

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

    pub fn new_face(&self, filepathname: &str, face_index: ffi::FT_Long) -> FtResult<Face> {
        unsafe {
            let mut face = std::ptr::null_mut();

            let path_str = filepathname.to_c_str();

            let err = ffi::FT_New_Face(self.raw, path_str.as_ptr(), face_index, &mut face);
            if err == ffi::FT_Err_Ok {
                Ok(Face::from_raw(self.raw, face))
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn new_memory_face(&self, buffer: &[u8], face_index: ffi::FT_Long) -> FtResult<Face> {
        unsafe {
            let mut face = std::ptr::null_mut();
            let err = ffi::FT_New_Memory_Face(self.raw, buffer.as_ptr(), buffer.len() as ffi::FT_Long, face_index, &mut face);
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
                std::io::println(format!("Failed to drop Library. Error Code: {}", err).as_slice());
            }
        }
    }
}

