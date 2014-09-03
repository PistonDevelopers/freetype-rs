#![feature(globs)]

extern crate graphics;
extern crate freetype;
extern crate piston;

extern crate sdl2_game_window;
extern crate opengl_graphics;

use std::collections::hashmap::HashMap;
use graphics::*;
use freetype as ft;
use freetype::Face;
use sdl2_game_window::GameWindowSDL2;
use opengl_graphics::{
    Gl,
    Texture,
};
use piston::{
    AssetStore,
    GameIterator,
    GameIteratorSettings,
    GameWindowSettings,
    Render,
};


struct Character {
    glyph: ft::Glyph,
    bitmap_glyph: ft::BitmapGlyph,
    texture: Texture,
}

fn render_text(buffer: &mut HashMap<char, Character>, face: &Face, gl: &mut Gl, c: &Context, text: &str) {
    let mut x = 0;
    let mut y = 0;
    for ch in text.chars() {
        if !buffer.contains_key(&ch) {
            load_character(buffer, face, ch);
            std::io::println(format!("Loaded char {}", ch).as_slice());
        }
        let character = buffer.get(&ch);

        c.trans((x + character.bitmap_glyph.left()) as f64, (y - character.bitmap_glyph.top()) as f64).image(&character.texture).rgb(0.0, 0.0, 0.0).draw(gl);

        // A 16.16 vector that gives the glyph's advance width.
        x += (character.glyph.advance().x >> 16) as i32;
        y += (character.glyph.advance().y >> 16) as i32;
    }

fn load_character(buffer: &mut HashMap<char, Character>, face: &Face, ch: char) {
    face.load_char(ch as u64, ft::face::Default).unwrap();
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
    let mut window = GameWindowSDL2::new(
        piston::shader_version::opengl::OpenGL_3_2,
        GameWindowSettings {
            title: "Font with Piston (Cached)".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true
        }
    );

    let asset_store = AssetStore::from_folder("../assets");
    let freetype = ft::Library::init().unwrap();
    let font = asset_store.path("Arial.ttf").unwrap();
    let face = freetype.new_face(font.as_str().unwrap(), 0).unwrap();
    face.set_pixel_sizes(0, 48).unwrap();

    let mut buffer = HashMap::<char, Character>::new();

    let game_iter_settings = GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
    };

    let ref mut gl = Gl::new();

    for e in GameIterator::new(&mut window, &game_iter_settings) {
        match e {
            Render(args) => {
                let c = Context::abs(args.width as f64, args.height as f64);
                c.rgb(1.0, 1.0, 1.0).draw(gl);
                render_text(&mut buffer, &face, gl, &c.trans(0.0, 100.0), "Hello Piston!");
                render_text(&mut buffer, &face, gl, &c.trans(0.0, 160.0), "Hello Piston!");
                render_text(&mut buffer, &face, gl, &c.trans(0.0, 220.0), "Hello Piston!");
            },
            _ => {},
        }
    }
}

    for e in piston::GameIterator::new(&mut window, &game_iter_settings) {
        match e {
            Render(_args) =>
                app.render(&_args),
            _ => {},
        }
    }
}
