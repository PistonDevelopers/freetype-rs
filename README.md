# freetype-rs [![Build Status](https://travis-ci.org/PistonDevelopers/freetype-rs.svg?branch=master)](https://travis-ci.org/PistonDevelopers/freetype-rs)

Rust bindings for FreeType library

## Requirements

  * *Rust*: We currently compile against the *master* branch. The releases on
    http://www.rust-lang.org tend to not work.
  * *Cargo*: We use Cargo to compile the project.
  * *FreeType2 development libraries*: install through your favourite package
    management tool, or via http://www.freetype.org/download.html

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
