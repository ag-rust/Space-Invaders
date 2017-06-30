use graphics::*;
use world::*;
use color::*;
use opengl_graphics::{GlGraphics};
use piston::input::*;
use std::sync::Arc;
use entity::Entity;
use drawing::Drawable;
use point::Point;
use opengl_graphics::glyph_cache::GlyphCache;

pub struct App<'a> {
    pub gl: GlGraphics,
    pub world: World,
    pub render_calls: u64,
    pub glyph_cache: GlyphCache<'a>,
}

impl<'a> App<'a> {
    pub fn render(&mut self, args: &RenderArgs) {
        if self.world.dead() {
            panic!("dead");
        }

        self.render_calls += 1;

        let world = &self.world;
        let fps = self.fps();
        let mut glyph_cache = &mut self.glyph_cache;
        let enemies = &self.world.enemies;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(Color::white().to_array(), gl);

            world.hero.draw_shape(world.config.world_size, &c, gl);
            fps.draw_text(world.config.world_size, &mut glyph_cache, &c, gl);

            for (position, enemy) in enemies {
                (position.clone(), enemy.clone()).draw_shape(world.config.world_size, &c, gl);
            }
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

    pub fn update(&mut self, args: &UpdateArgs) {
        self.world.move_enemies();
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

    fn fps(&self) -> FrameRate {
        let time = self.world.start_time.elapsed().as_secs();
        if time == 0 { return FrameRate { fps: 0 } }
        let fps = self.render_calls / self.world.start_time.elapsed().as_secs();
        FrameRate { fps: fps }
    }
}

pub struct FrameRate {
    pub fps: u64,
}
