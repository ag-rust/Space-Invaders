use graphics::*;
use game_state::*;
use world::*;
use color::*;
use opengl_graphics::{GlGraphics};
use piston::input::*;
use std::sync::Arc;
use entity::Entity;
use drawing::Drawable;
use point::Point;
use intro_text::*;
use opengl_graphics::glyph_cache::GlyphCache;

pub struct App<'a> {
    pub gl: GlGraphics,
    pub world: World,
    pub render_calls: u64,
    pub glyph_cache: GlyphCache<'a>,
}

impl<'a> App<'a> {
    pub fn render(&mut self, args: &RenderArgs) {
        self.render_calls += 1;

        let world = &self.world;
        let fps = self.fps();
        let mut glyph_cache = &mut self.glyph_cache;
        let enemies = &self.world.enemies;
        let projectiles = &self.world.projectiles;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(world.config.world_background_color.to_array(), gl);

            match world.game_state {
                GameState::Intro => {
                    let intro_text = IntroText::new("Welcome, press any button to start");
                    intro_text.draw_text(world.config.world_size, &mut glyph_cache, &c, gl);
                },
                GameState::Playing => {
                    world.hero.draw_shape(world.config.world_size, &c, gl);
                    for &map in [enemies, projectiles].iter() {
                        for (position, enemy) in map {
                            (position.clone(), enemy.clone())
                                .draw_shape(world.config.world_size, &c, gl);
                        }
                    }
                    world.score.draw_text(world.config.world_size, &mut glyph_cache, &c, gl);
                },
                GameState::Dead => {
                    let intro_text = IntroText::new("You died, press any button to start again");
                    intro_text.draw_text(world.config.world_size, &mut glyph_cache, &c, gl);
                },
                GameState::Won => {
                },
            }

            fps.draw_text(world.config.world_size, &mut glyph_cache, &c, gl);
        });
    }


    pub fn press(&mut self, args: &Button) {
        match self.world.game_state {
            GameState::Intro => {
                self.world.start();
            },
            GameState::Playing => {
                match args {
                    &Button::Keyboard(Key::Right) => self.world.move_right(),
                    &Button::Keyboard(Key::Left) => self.world.move_left(),
                    &Button::Keyboard(Key::Up) => self.world.move_up(),
                    &Button::Keyboard(Key::Down) => self.world.move_down(),
                    &Button::Keyboard(Key::Space) => self.world.shoot(),
                    _ => {},
                }
            },
            GameState::Dead => {
                self.world.start();
            },
            GameState::Won => {
                panic!("You won!");
            },
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        match self.world.game_state {
            GameState::Intro => {},
            GameState::Playing => {
                self.world.check_for_out_of_bounds_enemies();
                self.world.move_enemies();
                self.world.move_projectiles();
                self.world.check_for_projectile_enemy_collisions();
                self.world.check_if_won();
                self.world.check_if_still_alive();
                self.world.check_if_enemy_at_bottom();
            },
            GameState::Dead => {},
            GameState::Won => {},
        }
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
