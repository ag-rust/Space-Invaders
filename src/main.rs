extern crate rand;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod app;
mod color;
mod config;
mod default;
mod direction;
mod drawing;
mod entity;
mod game_state;
mod grid_distribution;
mod intro_text;
mod max_min;
mod point;
mod score;
mod size;
mod utils;
mod world;

use color::*;
use app::*;
use config::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::collections::HashMap;
use opengl_graphics::glyph_cache::GlyphCache;
use world::*;
use size::*;
use std::path::Path;

fn main() {
    let config = Config::default();

    let mut world = World::new(config);

    let opengl = OpenGL::V3_2;

    let font_path = Path::new("./assets/InputMono-Regular.ttf");
    let glyph_cache = GlyphCache::new(font_path).unwrap();

    let mut window: Window = WindowSettings::new(
            "Asteroids",
            [world.config.world_size.width, world.config.world_size.height]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        world: world,
        render_calls: 0,
        glyph_cache: glyph_cache,
    };

    let mut event_stream = Events::new(EventSettings::new());
    while let Some(input) = event_stream.next(&mut window) {
        match input {
            Input::Render(args) => { app.render(&args) },
            Input::Update(args) => { app.update(&args) },
            Input::Move(args) => { app.move_it(&args) },
            Input::AfterRender(args) => { app.after_render(&args) },
            Input::Close(args) => { app.close(&args) },
            Input::Cursor(args) => { app.cursor(&args) },
            Input::Focus(args) => { app.focus(&args) },
            Input::Idle(args) => { app.idle(&args) },
            Input::Press(args) => { app.press(&args) },
            Input::Release(args) => { app.release(&args) },
            Input::Resize(one, two) => { app.resize(&one, &two) },
            Input::Text(args) => { app.text(&args) },

            Input::Custom(_, _) => {},
        }
    }
}
