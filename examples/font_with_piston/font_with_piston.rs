#![feature(globs)]

extern crate graphics;
extern crate freetype;
extern crate piston;

extern crate sdl2_game_window;
extern crate opengl_graphics;

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
    GameWindowSettings,
    RenderArgs
};

#[allow(dead_code)]
pub struct App {
    freetype: ft::Library,
    face: ft::Face,

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

            gl: Gl::new(),
        }
    }

    pub fn render_text(&mut self, c: &Context, text: &str) {
        let mut x = 0;
        let mut y = 0;
        for ch in text.chars() {
            self.face.load_char(ch as u64, ft::face::Render).unwrap();
            let g = self.face.glyph();

            let texture = Texture::from_memory_alpha(g.bitmap().buffer(), g.bitmap().width() as u32, g.bitmap().rows() as u32).unwrap();
            c.trans((x + g.bitmap_left()) as f64, (y - g.bitmap_top()) as f64).image(&texture).rgb(0.0, 0.0, 0.0).draw(&mut self.gl);

            x += (g.advance().x >> 6) as i32;
            y += (g.advance().y >> 6) as i32;
        }
    }
}

impl Game for App {
    fn render(&mut self, args: &RenderArgs) {
        let c = Context::abs(args.width as f64, args.height as f64);
        c.rgb(1.0, 1.0, 1.0).draw(&mut self.gl);

        self.render_text(&c.trans(0.0, 100.0), "Hello Piston!");
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

