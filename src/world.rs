extern crate rand;

use std::collections::HashMap;
use color::Color;
use size::Size;
use entity::*;
use point::*;
use rand::*;
use std::time::{Instant};
use rand::distributions::{IndependentSample, Range};

pub struct World {
    pub size: Size,
    pub hero: (Point, Entity),
    pub enemies: HashMap<Point, Entity>,
    pub hero_speed: u32,
    pub start_time: Instant,
    pub alive: bool,
    pub rng: ThreadRng,
    pub number_of_enemies: u32,
    pub enemy_size: Size,
    pub starting_safe_zone_around_hero_size: u32,
}

// Initialization
impl World {
    pub fn new() -> World {
        let player = Entity {
            color: Color::green(),
            size: Size { width: 10, height: 10 },
        };

        World {
            size: Size { height: 0, width: 0 },
            hero: (Point::zero(), player),
            enemies: HashMap::new(),
            hero_speed: 1,
            start_time: Instant::now(),
            alive: true,
            rng: rand::thread_rng(),
            number_of_enemies: 0,
            enemy_size: Size::zero(),
            starting_safe_zone_around_hero_size: 0,
        }
    }

    pub fn add_enemy_at(&mut self, position: Point, enemy: Entity) {
        self.enemies.insert(position, enemy);
    }

    pub fn populate_with_enemies(&mut self) {
        let mut x_range = Range::new(
            0,
            self.size.width - self.enemy_size.width + 1);

        let mut y_range = Range::new(
            0,
            self.size.height - self.enemy_size.height + 1);

        let h = PositionAndSize {
            position: self.hero.0,
            size: self.hero.1.size,
        };

        for _ in (1..self.number_of_enemies + 1) {
            let enemy = Entity {
                size: Size { height: self.enemy_size.height, width: self.enemy_size.width },
                color: Color::red(),
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
                x += self.hero.1.size.width + self.starting_safe_zone_around_hero_size;
                y += self.hero.1.size.height + self.starting_safe_zone_around_hero_size;
                pos = Point { x: x, y: y };
            }

            self.add_enemy_at(pos, enemy);
        }
    }
}

// Moving the hero
impl World {
    pub fn move_up(&mut self) {
        let (current_pos, _) = self.hero;
        if current_pos.y <= 0 { return }
        let new_pos = Point { y: current_pos.y - self.hero_speed, .. current_pos };
        self.hero.0 = new_pos;
        self.check_if_still_alive();
    }

    pub fn move_down(&mut self) {
        let (current_pos, _) = self.hero;
        if current_pos.y + self.hero.1.size.height >= self.size.height { return }
        let new_pos = Point { y: current_pos.y + self.hero_speed, .. current_pos };
        self.hero.0 = new_pos;
        self.check_if_still_alive();
    }

    pub fn move_left(&mut self) {
        let (current_pos, _) = self.hero;
        if current_pos.x <= 0 { return }
        let new_pos = Point { x: current_pos.x - self.hero_speed , .. current_pos };
        self.hero.0 = new_pos;
        self.check_if_still_alive();
    }

    pub fn move_right(&mut self) {
        let (current_pos, _) = self.hero;
        if current_pos.x + self.hero.1.size.width >= self.size.width { return }
        let new_pos = Point { x: current_pos.x + self.hero_speed , .. current_pos };
        self.hero.0 = new_pos;
        self.check_if_still_alive();
    }

    pub fn check_if_still_alive(&mut self) {
        let h = PositionAndSize {
            position: self.hero.0,
            size: self.hero.1.size,
        };

        // TODO: Can this be made O(1)?
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
        let enemy_speed = 1;

        let mut new_enemy_positions = HashMap::new();

        for (position, enemy) in &self.enemies {
            let mut x_range = Range::new(-1, 2); // exclusive range
            let mut y_range = Range::new(-1, 2); // exclusive range

            let mut x_diff = x_range.ind_sample(&mut self.rng);
            let mut y_diff = y_range.ind_sample(&mut self.rng);

            let x_new = ((position.x as i32 + x_diff, 0 as i32).max(), self.size.width as i32).min();
            let y_new = ((position.y as i32 + y_diff, 0 as i32).max(), self.size.height as i32).min();

            let pos = Point { x: x_new as u32, y: y_new as u32 };

            new_enemy_positions.insert(pos, enemy.clone());
        }

        self.enemies = new_enemy_positions;

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
