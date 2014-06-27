freetype-rs
===========

Rust bindings for FreeType library

Requirements
------------

  * *Rust*: We currently compile against the *master* branch. The releases on http://www.rust-lang.org tend to not work.
  * *FreeType2 development libraries*: install through your favourite package management tool, or via http://www.freetype.org/download.html

Build
-----
Clone this repo then run
```
make
```

Examples
--------
To build examples, you need following dependences

  * [rust-graphics](https://github.com/PistonDevelopers/rust-graphics)
  * [rust-image](https://github.com/PistonDevelopers/rust-image)
  * [piston](https://github.com/PistonDevelopers/piston)
  * [gl-rs](https://github.com/bjz/gl-rs)
  * [opengl_graphics](https://github.com/PistonDevelopers/opengl_graphics)
  * [rust-sdl2](https://github.com/AngryLawyer/rust-sdl2)
  * [sdl2_game_window](https://github.com/PistonDevelopers/sdl2_game_window)

Clone and compile all the dependences. Put/Link the .rlib in the folder `target/deps` then run
```
make examples
```

