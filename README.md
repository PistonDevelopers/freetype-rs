# freetype-rs [![Build Status](https://travis-ci.org/PistonDevelopers/freetype-rs.svg?branch=master)](https://travis-ci.org/PistonDevelopers/freetype-rs)

Rust bindings for FreeType library

## Requirements

  * *Cargo*: We use Cargo to compile the project.
  * *FreeType2 development libraries*: For installation instructions see
    [freetype-sys](https://github.com/PistonDevelopers/freetype-sys).

If the building fails, then it is likely that pkg-config does not find a FreeType library it can bind to. Use the feature "bundled" to build a static version of the library (requires a C compiler):

```text
[dependencies]
freetype-rs = { version = "*", features = ["bundled"] }
```

## Build

Clone this repo then run
```
cd freetype-rs
cargo build
```

## Examples

To build examples, use `cargo test`. They are all built in `./target/debug/examples/*`.

To run examples, use `cargo run --example name`, for example:
```
cargo run --example single_glyph examples/assets/FiraSans-Regular.ttf A
```

[How to contribute](https://github.com/PistonDevelopers/piston/blob/master/CONTRIBUTING.md)
