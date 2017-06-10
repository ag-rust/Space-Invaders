mod world;
mod color;
mod size;
mod entity;
mod point;
mod app;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use color::*;
use app::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::collections::HashMap;
use world::*;

fn main() {
    let mut world = World::new();
    world.size.height = 50;
    world.size.width = 50;

    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [world.size.height, world.size.width]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        world: world,
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
