# freetype-rs [![Build Status](https://travis-ci.org/PistonDevelopers/freetype-rs.svg)](https://travis-ci.org/PistonDevelopers/freetype-rs)

Rust bindings for FreeType library

## Requirements

  * *Rust*: We currently compile against the *master* branch. The releases on http://www.rust-lang.org tend to not work.
  * *Cargo*: We use Cargo to compile the project.
  * *FreeType2 development libraries*: install through your favourite package management tool, or via http://www.freetype.org/download.html

## Build

Clone this repo then run
```
cd freetype-rs
cargo build
```

## Examples

To build examples, enter the corresponding example folder and run `cargo build`.

For example:
```
# build
cd examples/font_with_piston
cargo build

# run
./target/font_with_piston
```
