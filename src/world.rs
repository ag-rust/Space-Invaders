extern crate rand;

use std::collections::HashMap;
use color::Color;
use size::Size;
use entity::*;
use point::*;
use config::*;
use score::*;
use rand::*;
use default;
use std::time::{Instant};
use rand::distributions::{IndependentSample, Range};

pub struct World {
    pub hero: (Point, Entity),
    pub enemies: HashMap<Point, Entity>,
    pub start_time: Instant,
    pub alive: bool,
    pub rng: ThreadRng,
    pub config: Config,
    pub score: Score,
}

// Initialization
impl World {
    pub fn new(config: Config) -> World {
        let hero = Entity {
            color: config.hero_color,
            size: config.hero_size,
        };

        let world = World {
            hero: (config.hero_starting_position, hero),
            enemies: HashMap::new(),
            start_time: Instant::now(),
            alive: true,
            rng: rand::thread_rng(),
            config: config,
            score: default::default(),
        };

        world
    }

    pub fn add_enemy_at(&mut self, position: Point, enemy: Entity) {
        self.enemies.insert(position, enemy);
    }

    pub fn populate_with_enemies(&mut self) {
        let mut x_range = Range::new(
            0,
            self.config.world_size.width - self.config.enemy_size.width + 1);

        let mut y_range = Range::new(
            0,
            self.config.world_size.height - self.config.enemy_size.height + 1);

        let h = PositionAndSize {
            position: self.hero.0,
            size: self.hero.1.size,
        };

        for _ in (1..self.config.number_of_enemies + 1) {
            let enemy = Entity {
                size: Size {
                    height: self.config.enemy_size.height,
                    width: self.config.enemy_size.width
                },
                color: self.config.enemy_color,
            };

            let mut x = x_range.ind_sample(&mut self.rng);
            let mut y = y_range.ind_sample(&mut self.rng);
            let mut pos = Point { x: x, y: y };

            let p = PositionAndSize {
                position: pos,
                size: enemy.size,
            };

            // make sure an enemy doesn't spawn on top of the hero
            if h.collides_with(&p) || p.collides_with(&h) {
                x += self.hero.1.size.width + self.config.starting_safe_zone_around_hero_size;
                y += self.hero.1.size.height + self.config.starting_safe_zone_around_hero_size;
                pos = Point { x: x, y: y };
            }

            self.add_enemy_at(pos, enemy);
        }
    }
}

// Moving the hero
impl World {
    pub fn move_up(&mut self) {
        if !self.config.vertical_movement_allowed { return }
        let (current_pos, _) = self.hero;
        if current_pos.y <= 0 { return }
        let new_pos = Point { y: current_pos.y - self.config.hero_speed, .. current_pos };
        self.hero.0 = new_pos;
        self.check_if_still_alive();
    }

    pub fn move_down(&mut self) {
        if !self.config.vertical_movement_allowed { return }
        let (current_pos, _) = self.hero;
        if current_pos.y + self.hero.1.size.height >= self.config.world_size.height { return }
        let new_pos = Point { y: current_pos.y + self.config.hero_speed, .. current_pos };
        self.hero.0 = new_pos;
        self.check_if_still_alive();
    }

    pub fn move_left(&mut self) {
        if !self.config.horizontal_movement_allowed { return }
        let (current_pos, _) = self.hero;
        if current_pos.x <= 0 { return }
        let new_pos = Point { x: current_pos.x - self.config.hero_speed , .. current_pos };
        self.hero.0 = new_pos;
        self.check_if_still_alive();
    }

    pub fn move_right(&mut self) {
        if !self.config.horizontal_movement_allowed { return }
        let (current_pos, _) = self.hero;
        if current_pos.x + self.hero.1.size.width >= self.config.world_size.width { return }
        let new_pos = Point { x: current_pos.x + self.config.hero_speed , .. current_pos };
        self.hero.0 = new_pos;
        self.check_if_still_alive();
    }

    pub fn check_if_still_alive(&mut self) {
        if !self.alive { return };

        let h = PositionAndSize {
            position: self.hero.0,
            size: self.hero.1.size,
        };

        for (position, enemy) in &self.enemies {
            let p = PositionAndSize {
                position: position.clone(),
                size: enemy.size,
            };

            if h.collides_with(&p) || p.collides_with(&h) {
                self.alive = false;
                return;
            }
        }
    }

    pub fn dead(&self) -> bool { !self.alive }

    pub fn move_enemies(&mut self) {
        self.check_if_still_alive();
    }
}

trait MaxMin<T> where T: PartialOrd + Copy {
    fn max(&self) -> T;
    fn min(&self) -> T;
}

impl<T> MaxMin<T> for (T, T) where T: PartialOrd + Copy {
    fn max(&self) -> T {
        if self.0 > self.1 { self.0 } else { self.1 }
    }

    fn min(&self) -> T {
        if self.0 < self.1 { self.0 } else { self.1 }
    }
}
