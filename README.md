# freetype-rs [![Build Status](https://travis-ci.org/PistonDevelopers/freetype-rs.svg?branch=master)](https://travis-ci.org/PistonDevelopers/freetype-rs)

Rust bindings for FreeType library

## Requirements

### Linux

Install FreeType via your package manager. For example in Debian-based distributions:

```
sudo apt install libfreetype6-dev
```

### Mac

Use [Homebrew](http://brew.sh/) to install FreeType:

```
brew install freetype
```

### Windows

Install [MSYS2](http://sourceforge.net/p/msys2/wiki/MSYS2%20installation/) and run one of the
following two commands depending on your Rust version in a MSYS2 Shell:

```sh
pacman -S mingw-w64-i686-freetype   # 32 bit Rust
pacman -S mingw-w64-x86_64-freetype # 64 bit Rust
```

Now add `C:\msys32\mingw32\bin` or `C:\msys64\mingw64\bin` to your PATH and copy the
`libfreetype-6.dll` file from that folder to
`<Rust's install path>\bin\rustlib\<arch>-pc-windows-gnu\lib`.

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
