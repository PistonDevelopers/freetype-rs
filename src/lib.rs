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
//! ```ignore
//! extern crate freetype;
//!
//! fn main() {
//!     use freetype::Library;
//!
//!     // Init the library
//!     let lib = Library::init().unwrap();
//!     // Load a font face
//!     let face = lib.new_face("/path/to/a/font/file.ttf", 0).unwrap();
//!     // Set the font size
//!     face.set_char_size(40 * 64, 0, 50, 0).unwrap();
//!     // Load a character
//!     face.load_char('A' as usize, freetype::face::RENDER).unwrap();
//!     // Get the glyph instance
//!     let glyph = face.glyph();
//!     do_something_with_bitmap(glyph.bitmap());
//! }
//! ```
//!
//! See in the `examples/` folder for more examples.
//!
//! # External links
//! - See [freetype docs](http://www.freetype.org/freetype2/docs/reference/ft2-index.html)
//!   for more information

#![deny(missing_copy_implementations)]

#[macro_use]
extern crate bitflags;
extern crate libc;
pub extern crate freetype_sys;

pub use bitmap::Bitmap;
pub use bitmap_glyph::BitmapGlyph;
pub use error::{ FtResult, Error };
pub use face::Face;
pub use glyph::Glyph;
pub use glyph_slot::GlyphSlot;
pub use library::{ Library, LcdFilter };
pub use outline::Outline;
pub use render_mode::RenderMode;
pub use stroker::{Stroker, StrokerLineCap, StrokerLineJoin };
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
pub mod stroker;
pub mod tt_os2;
pub mod tt_postscript;

pub type BBox = ffi::FT_BBox;
pub type GlyphMetrics = ffi::FT_Glyph_Metrics;
pub type Matrix = ffi::FT_Matrix;
pub type Vector = ffi::FT_Vector;
