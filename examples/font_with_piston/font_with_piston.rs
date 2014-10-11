#![feature(globs)]

extern crate graphics;
extern crate freetype;
extern crate piston;

extern crate sdl2_game_window;
extern crate opengl_graphics;

use piston::graphics::*;
use freetype as ft;
use freetype::Face;
use sdl2_game_window::WindowSDL2;
use opengl_graphics::{
    Gl,
    Texture,
};
use piston::{
    AssetStore,
    EventIterator,
    EventSettings,
    WindowSettings,
    Render,
};

fn render_text(face: &Face, gl: &mut Gl, c: &Context, text: &str) {
    let mut x = 0;
    let mut y = 0;
    for ch in text.chars() {
        face.load_char(ch as u64, ft::face::Render).unwrap();
        let g = face.glyph();

        let texture = Texture::from_memory_alpha(g.bitmap().buffer(), g.bitmap().width() as u32, g.bitmap().rows() as u32).unwrap();
        c.trans((x + g.bitmap_left()) as f64, (y - g.bitmap_top()) as f64).image(&texture).rgb(0.0, 0.0, 0.0).draw(gl);

        x += (g.advance().x >> 6) as i32;
        y += (g.advance().y >> 6) as i32;
    }
}

fn main() {
    let opengl = piston::shader_version::opengl::OpenGL_3_2;
    let mut window = WindowSDL2::new(
        piston::shader_version::opengl::OpenGL_3_2,
        WindowSettings {
            title: "Font with Piston".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let asset_store = AssetStore::from_folder("../assets");
    let freetype = ft::Library::init().unwrap();
    let font = asset_store.path("Arial.ttf").unwrap();
    let face = freetype.new_face(font.as_str().unwrap(), 0).unwrap();
    face.set_pixel_sizes(0, 48).unwrap();

    let event_settings = EventSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
    };

    let ref mut gl = Gl::new(opengl);

    for e in EventIterator::new(&mut window, &event_settings) {
        match e {
            Render(args) => {
                let c = Context::abs(args.width as f64, args.height as f64);
                c.rgb(1.0, 1.0, 1.0).draw(gl);
                render_text(&face, gl, &c.trans(0.0, 100.0), "Hello Piston!");
            },
            _ => {},
        }
    }
}

