# freetype-rs [![Build Status](https://travis-ci.org/PistonDevelopers/freetype-rs.svg?branch=master)](https://travis-ci.org/PistonDevelopers/freetype-rs)

Rust bindings for FreeType library

[Documentation](http://rust-ci.org/PistonDevelopers/freetype-rs/doc/freetype/)

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

To build examples, use `cargo test`. They are all built in `./target/examples/*`.

For example:
```
# build
cargo test

# run
./target/examples/font_with_piston
```

[How to contribute](https://github.com/PistonDevelopers/piston/blob/master/CONTRIBUTING.md)
