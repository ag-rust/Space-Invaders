use std::collections::HashMap;
use color::Color;
use size::Size;
use entity::*;
use point::*;
use std::time::{Instant};

#[derive(Debug)]
pub struct World {
    pub size: Size,
    pub hero: (Point, Entity),
    pub enemies: HashMap<Point, Entity>,
    pub hero_speed: u32,
    pub start_time: Instant,
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
        }
    }

    pub fn add_enemy_at(&mut self, position: Point, enemy: Entity) {
        self.enemies.insert(position, enemy);
    }
}

// Moving the hero
impl World {
    pub fn move_up(&mut self) {
        let (current_pos, _) = self.hero;
        if current_pos.y <= 0 { return }
        let new_pos = Point { y: current_pos.y - self.hero_speed, .. current_pos };
        self.hero.0 = new_pos;
    }

    pub fn move_down(&mut self) {
        let (current_pos, _) = self.hero;
        if current_pos.y + self.hero.1.size.height >= self.size.height { return }
        let new_pos = Point { y: current_pos.y + self.hero_speed, .. current_pos };
        self.hero.0 = new_pos;
    }

    pub fn move_left(&mut self) {
        let (current_pos, _) = self.hero;
        if current_pos.x <= 0 { return }
        let new_pos = Point { x: current_pos.x - self.hero_speed , .. current_pos };
        self.hero.0 = new_pos;
    }

    pub fn move_right(&mut self) {
        let (current_pos, _) = self.hero;
        if current_pos.x + self.hero.1.size.width >= self.size.width { return }
        let new_pos = Point { x: current_pos.x + self.hero_speed , .. current_pos };
        self.hero.0 = new_pos;
    }
}
