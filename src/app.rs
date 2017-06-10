use graphics::*;
use world::*;
use opengl_graphics::{GlGraphics};
use piston::input::*;
use std::sync::Arc;

pub struct App {
    pub gl: GlGraphics,
    pub world: World
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
    }

    pub fn update(&mut self, args: &UpdateArgs) {
    }

    pub fn press(&mut self, args: &Button) {
    }

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
