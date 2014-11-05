
#![crate_type = "lib"]
#![crate_name="freetype"]
#![desc = "Rust bindings for FreeType"]
#![license = "MIT"]

extern crate libc;

pub use bitmap::Bitmap;
pub use bitmap_glyph::BitmapGlyph;
pub use error::FtResult;
pub use face::Face;
pub use glyph::Glyph;
pub use glyph_slot::GlyphSlot;
pub use library::Library;
pub use outline::Outline;
pub use render_mode::RenderMode;

pub mod bitmap;
pub mod bitmap_glyph;
pub mod error;
pub mod face;
pub mod ffi;
pub mod glyph;
pub mod glyph_slot;
pub mod library;
pub mod outline;
pub mod render_mode;

pub type BBox = ffi::FT_BBox;
pub type GlyphMetrics = ffi::FT_Glyph_Metrics;
pub type Matrix = ffi::FT_Matrix;
pub type Vector = ffi::FT_Vector;

