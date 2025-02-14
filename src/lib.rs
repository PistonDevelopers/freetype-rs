//!
//! Rust wrapper around freetype 2 library
//!
//! # Initialization
//!
//! To create a new freetype context, instantiate the Library struct as below.
//! The Library (along with other objects) obeys RAII and is dropped when the struct goes out of
//! scope.
//!
//! # Example
//!
//! ```no_run
//! fn main() {
//!     use freetype::Library;
//!     use freetype::face::LoadFlag;
//!
//!     // Init the library
//!     let lib = Library::init().unwrap();
//!     // Load a font face
//!     let face = lib.new_face("/path/to/a/font/file.ttf", 0).unwrap();
//!     // Set the font size
//!     face.set_char_size(40 * 64, 0, 50, 0).unwrap();
//!     // Load a character
//!     face.load_char('A' as usize, LoadFlag::RENDER).unwrap();
//!     // Get the glyph instance
//!     let glyph = face.glyph();
//!     do_something_with_bitmap(glyph.bitmap());
//! }
//! # fn do_something_with_bitmap(_bitmap: freetype::Bitmap) {}
//! ```
//!
//! See in the `examples/` folder for more examples.
//!
//! # External links
//! - See [freetype docs](http://www.freetype.org/freetype2/docs/reference/ft2-index.html)
//!   for more information

#![allow(clippy::missing_safety_doc)]
#![deny(missing_copy_implementations)]

pub use freetype_sys;

pub use crate::bitmap::Bitmap;
pub use crate::bitmap_glyph::BitmapGlyph;
pub use crate::error::{Error, FtResult};
pub use crate::face::Face;
pub use crate::glyph::Glyph;
pub use crate::glyph_slot::GlyphSlot;
pub use crate::library::{LcdFilter, Library};
pub use crate::outline::Outline;
pub use crate::render_mode::RenderMode;
pub use crate::stroker::{Stroker, StrokerLineCap, StrokerLineJoin};
pub use freetype_sys as ffi;

pub mod bitmap;
pub mod bitmap_glyph;
pub mod charmap;
pub mod error;
pub mod face;
pub mod glyph;
pub mod glyph_slot;
pub mod library;
pub mod outline;
pub mod render_mode;
pub mod stroker;
pub mod tt_os2;
pub mod tt_postscript;

pub type BBox = ffi::FT_BBox;
pub type GlyphMetrics = ffi::FT_Glyph_Metrics;
pub type Matrix = ffi::FT_Matrix;
pub type Vector = ffi::FT_Vector;
