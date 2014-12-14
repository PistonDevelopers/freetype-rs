#![feature(globs)]

extern crate graphics;
extern crate freetype;
extern crate shader_version;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate event;

use std::cell::RefCell;
use freetype as ft;
use freetype::Face;
use sdl2_window::Sdl2Window;
use opengl_graphics::{
    Gl,
    Texture,
};
use event::{
    Event,
    Events,
    WindowSettings,
};

fn render_text(face: &mut Face, gl: &mut Gl, c: &graphics::Context, text: &str) {
    use graphics::*;

    let mut x = 0;
    let mut y = 0;
    for ch in text.chars() {
        face.load_char(ch as u64, ft::face::RENDER).unwrap();
        let g = face.glyph();

        let texture = Texture::from_memory_alpha(g.bitmap().buffer(), g.bitmap().width() as u32, g.bitmap().rows() as u32).unwrap();
        graphics::Image::colored(graphics::color::BLACK).draw(
            &texture, 
            &c.trans((x + g.bitmap_left()) as f64, (y - g.bitmap_top()) as f64),
            gl
        );

        x += (g.advance().x >> 6) as i32;
        y += (g.advance().y >> 6) as i32;
    }
}

fn main() {
    let opengl = shader_version::OpenGL::_3_2;
    let window = Sdl2Window::new(
        opengl,
        WindowSettings {
            title: "Font with Piston".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let freetype = ft::Library::init().unwrap();
    let font = Path::new("./assets/Arial.ttf");
    let mut face = freetype.new_face(font.as_str().unwrap(), 0).unwrap();
    face.set_pixel_sizes(0, 48).unwrap();

    let ref mut gl = Gl::new(opengl);

    let window = RefCell::new(window);
    for e in Events::new(&window) {
        match e {
            Event::Render(args) => {
                use graphics::*;

                let c = Context::abs(args.width as f64, args.height as f64);
                graphics::clear([1.0, 1.0, 1.0, 1.0], gl);
                render_text(&mut face, gl, &c.trans(0.0, 100.0), "Hello Piston!");
            },
            _ => {},
        }
    }
}

