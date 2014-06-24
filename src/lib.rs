
#![crate_type = "lib"]
#![crate_id="freetype#0.0.1"]
#![desc = "Rust bindings for FreeType, based on version 2.5.2"]
#![license = "MIT"]

#![feature(globs)]

extern crate libc;

pub use library::Library;
pub use face::Face;
pub use glyph_slot::GlyphSlot;

pub mod ffi;
pub mod library;
pub mod face;
pub mod glyph_slot;
pub mod error;

pub type FtResult<T> = Result<T, error::Error>;
pub type BBox = ffi::FT_BBox;
pub type Bitmap = ffi::FT_Bitmap;
pub type Matrix = ffi::FT_Matrix;
pub type Vector = ffi::FT_Vector;
pub type GlyphMetrics = ffi::FT_Glyph_Metrics;

