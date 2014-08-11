
#![feature(globs)]

extern crate graphics;
extern crate freetype;
extern crate piston;

extern crate sdl2_game_window;
extern crate opengl_graphics;

use std::collections::hashmap::HashMap;
use graphics::*;
use ft = freetype;
use sdl2_game_window::GameWindowSDL2;
use opengl_graphics::{
    Gl,
    Texture,
};
use piston::{
    AssetStore,
    Game,
    GameIteratorSettings,
    GameWindow,
    GameWindowSettings,
    RenderArgs
};


struct Character {
    glyph: ft::Glyph,
    bitmap_glyph: ft::BitmapGlyph,
    texture: Texture,
}

#[allow(dead_code)]
pub struct App {
    freetype: ft::Library,
    face: ft::Face,

    buffer: HashMap<char, Character>,

    gl: Gl,
}

impl App {
    /// Creates a new application.
    pub fn new() -> App {
        let freetype = ft::Library::init().unwrap();
        let asset_store = AssetStore::from_folder("../assets");
        let font = asset_store.path("Arial.ttf").unwrap();
        let face = freetype.new_face(font.as_str().unwrap(), 0).unwrap();
        face.set_pixel_sizes(0, 48).unwrap();

        App {
            freetype: freetype,
            face: face,

            buffer: HashMap::new(),

            gl: Gl::new(),
        }
    }

    pub fn render_text(&mut self, c: &Context, text: &str) {
        let mut x = 0;
        let mut y = 0;
        for ch in text.chars() {
            if !self.buffer.contains_key(&ch) {
                self.load_character(ch);
                std::io::println(format!("Loaded char {}", ch).as_slice());
            }
            let character = self.buffer.get(&ch);

            c.trans((x + character.bitmap_glyph.left()) as f64, (y - character.bitmap_glyph.top()) as f64).image(&character.texture).rgb(0.0, 0.0, 0.0).draw(&mut self.gl);

            // A 16.16 vector that gives the glyph's advance width.
            x += (character.glyph.advance().x >> 16) as i32;
            y += (character.glyph.advance().y >> 16) as i32;
        }
    }

    fn load_character(&mut self, ch: char) {
        self.face.load_char(ch as u64, ft::face::Default).unwrap();
        let glyph = self.face.glyph().get_glyph().unwrap();
        let bitmap_glyph = glyph.to_bitmap(ft::render_mode::Normal, None).unwrap();
        let bitmap = bitmap_glyph.bitmap();
        let texture = Texture::from_memory_alpha(bitmap.buffer(), bitmap.width() as u32, bitmap.rows() as u32).unwrap();

        self.buffer.insert(ch, Character {
            glyph: glyph,
            bitmap_glyph: bitmap_glyph,
            texture: texture,
        });
    }
}

impl<W: GameWindow> Game<W> for App {
    fn render(&mut self, _window: &mut W, args: &RenderArgs) {
        let c = Context::abs(args.width as f64, args.height as f64);
        c.rgb(1.0, 1.0, 1.0).draw(&mut self.gl);

        self.render_text(&c.trans(0.0, 100.0), "Hello Piston!");
        self.render_text(&c.trans(0.0, 160.0), "Hello Piston!");
        self.render_text(&c.trans(0.0, 220.0), "Hello Piston!");
    }
}

fn main() {
    let mut window = GameWindowSDL2::new(
        GameWindowSettings {
            title: "Test".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true
        }
    );

    let game_iter_settings = GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    App::new().run(&mut window, &game_iter_settings);
}

