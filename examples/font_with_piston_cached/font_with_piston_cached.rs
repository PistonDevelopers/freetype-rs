#![feature(globs)]

extern crate shader_version;
extern crate graphics;
extern crate freetype;
extern crate event;
extern crate sdl2_window;
extern crate opengl_graphics;

use graphics::Context;
use std::collections::hash_map::HashMap;
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

struct Character {
    glyph: ft::Glyph,
    bitmap_glyph: ft::BitmapGlyph,
    texture: Texture,
}

fn render_text(buffer: &mut HashMap<char, Character>, face: &mut Face, gl: &mut Gl, c: &Context, text: &str) {
    use graphics::*;

    let mut x = 0;
    let mut y = 0;
    for ch in text.chars() {
        if !buffer.contains_key(&ch) {
            load_character(buffer, face, ch);
            std::io::println(format!("Loaded char {}", ch).as_slice());
        }
        let character = buffer.get(&ch).unwrap();

        c.trans((x + character.bitmap_glyph.left()) as f64, (y - character.bitmap_glyph.top()) as f64).image(&character.texture).rgb(0.0, 0.0, 0.0).draw(gl);

        // A 16.16 vector that gives the glyph's advance width.
        x += (character.glyph.advance().x >> 16) as i32;
        y += (character.glyph.advance().y >> 16) as i32;
    }
}

fn load_character(buffer: &mut HashMap<char, Character>, face: &mut Face, ch: char) {
    face.load_char(ch as u64, ft::face::DEFAULT).unwrap();
    let glyph = face.glyph().get_glyph().unwrap();
    let bitmap_glyph = glyph.to_bitmap(ft::render_mode::Normal, None).unwrap();
    let bitmap = bitmap_glyph.bitmap();
    let texture = Texture::from_memory_alpha(bitmap.buffer(), bitmap.width() as u32, bitmap.rows() as u32).unwrap();

    buffer.insert(ch, Character {
        glyph: glyph,
        bitmap_glyph: bitmap_glyph,
        texture: texture,
    });
}

fn main() {
    let opengl = shader_version::opengl::OpenGL::OpenGL_3_2;
    let window = Sdl2Window::new(
        opengl,
        WindowSettings {
            title: "Font with Piston (Cached)".to_string(),
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

    let mut buffer = HashMap::<char, Character>::new();

    let ref mut gl = Gl::new(opengl);

    for e in Events::new(window) {
        match e {
            Event::Render(args) => {
                use graphics::*;

                let c = Context::abs(args.width as f64, args.height as f64);
                c.rgb(1.0, 1.0, 1.0).draw(gl);
                render_text(&mut buffer, &mut face, gl, &c.trans(0.0, 100.0), "Hello Piston!");
                render_text(&mut buffer, &mut face, gl, &c.trans(0.0, 160.0), "Hello Piston!");
                render_text(&mut buffer, &mut face, gl, &c.trans(0.0, 220.0), "Hello Piston!");
            },
            _ => {},
        }
    }
}

