use std::collections::HashMap;
use color::Color;
use size::Size;
use entity::*;
use point::*;

#[derive(Debug)]
pub struct World {
    pub size: Size,
    pub hero: (Point, Entity),
    pub enemies: HashMap<Point, Entity>,
}

impl World {
    pub fn new() -> World {
        let player = Entity {
            color: Color::black(),
            size: Size { width: 10, height: 10 },
        };

        World {
            size: Size { height: 0, width: 0 },
            hero: (Point::zero(), player),
            enemies: HashMap::new(),
        }
    }

    pub fn add_enemy_at(&mut self, position: Point, enemy: Entity) {
        self.enemies.insert(position, enemy);
    }

    pub fn move_up(&mut self) {
    }

    pub fn move_down(&mut self) {
    }

    pub fn move_left(&mut self) {
    }

    pub fn move_right(&mut self) {
    }
}
