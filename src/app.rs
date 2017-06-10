use graphics::*;
use world::*;
use color::*;
use opengl_graphics::{GlGraphics};
use piston::input::*;
use std::sync::Arc;
use entity::Entity;
use drawing::Drawable;

pub struct App {
    pub gl: GlGraphics,
    pub world: World
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        let world = &self.world;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(Color::white().to_array(), gl);

            world.hero.draw(&c, gl);
        });
    }


    pub fn press(&mut self, args: &Button) {
        match args {
            &Button::Keyboard(Key::Right) => self.world.move_right(),
            &Button::Keyboard(Key::Left) => self.world.move_left(),
            &Button::Keyboard(Key::Up) => self.world.move_up(),
            &Button::Keyboard(Key::Down) => self.world.move_down(),
            _ => {},
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {}
    pub fn move_it(&mut self, args: &Motion) {}
    pub fn after_render(&mut self, args: &AfterRenderArgs) {}
    pub fn close(&mut self, args: &CloseArgs) {}
    pub fn cursor(&mut self, args: &bool) {}
    pub fn focus(&mut self, args: &bool) {}
    pub fn idle(&mut self, args: &IdleArgs) {}
    pub fn release(&mut self, args: &Button) {}
    pub fn resize(&mut self, foo: &u32, bar: &u32) {}
    pub fn text(&mut self, args: &String) {}

    // NOTE: Input::Custom is not handled
}
