
#![crate_type = "lib"]
#![crate_id="freetype#0.0.1"]
#![desc = "Rust bindings for FreeType, based on version 2.5.2"]
#![license = "MIT"]

#![feature(globs)]

extern crate libc;

pub use bitmap::Bitmap;
pub use bitmap_glyph::BitmapGlyph;
pub use face::Face;
pub use glyph::Glyph;
pub use glyph_slot::GlyphSlot;
pub use library::Library;

pub mod bitmap;
pub mod bitmap_glyph;
pub mod error;
pub mod face;
pub mod ffi;
pub mod glyph;
pub mod glyph_slot;
pub mod library;

pub type BBox = ffi::FT_BBox;
pub type FtResult<T> = Result<T, error::Error>;
pub type GlyphMetrics = ffi::FT_Glyph_Metrics;
pub type Matrix = ffi::FT_Matrix;
pub type Vector = ffi::FT_Vector;

