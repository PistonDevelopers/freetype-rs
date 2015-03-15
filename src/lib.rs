#![crate_type = "lib"]
#![deny(missing_copy_implementations)]
#![deny(raw_pointer_derive)]
#![feature(core, std_misc)]

#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate "freetype-sys" as freetype_sys;

pub use bitmap::Bitmap;
pub use bitmap_glyph::BitmapGlyph;
pub use error::FtResult;
pub use face::Face;
pub use glyph::Glyph;
pub use glyph_slot::GlyphSlot;
pub use library::Library;
pub use outline::Outline;
pub use render_mode::RenderMode;
pub use freetype_sys as ffi;

pub mod bitmap;
pub mod bitmap_glyph;
pub mod error;
pub mod face;
pub mod glyph;
pub mod glyph_slot;
pub mod library;
pub mod outline;
pub mod render_mode;
pub mod tt_os2;

pub type BBox = ffi::FT_BBox;
pub type GlyphMetrics = ffi::FT_Glyph_Metrics;
pub type Matrix = ffi::FT_Matrix;
pub type Vector = ffi::FT_Vector;

