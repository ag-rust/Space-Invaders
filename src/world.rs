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

// Initialization
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
}

// Moving the hero
impl World {
    pub fn move_up(&mut self) {
        let (current_pos, _) = self.hero;
        let new_pos = Point { y: current_pos.y - 1, .. current_pos };

        if self.hero_within_bounds_at(new_pos) {
            self.hero.0 = new_pos;
        }
    }

    pub fn move_down(&mut self) {
        let (current_pos, _) = self.hero;
        let new_pos = Point { y: current_pos.y + 1, .. current_pos };

        if self.hero_within_bounds_at(new_pos) {
            self.hero.0 = new_pos;
        }
    }

    pub fn move_left(&mut self) {
        let (current_pos, _) = self.hero;
        let new_pos = Point { y: current_pos.x - 1 , .. current_pos };

        if self.hero_within_bounds_at(new_pos) {
            self.hero.0 = new_pos;
        }
    }

    pub fn move_right(&mut self) {
        let (current_pos, _) = self.hero;
        let new_pos = Point { y: current_pos.x + 1 , .. current_pos };

        if self.hero_within_bounds_at(new_pos) {
            self.hero.0 = new_pos;
        }
    }

    fn hero_within_bounds_at(&self, new_pos: Point) -> bool {
        let hero = PositionAndSize {
            size: self.hero_size(),
            position: new_pos,
        };
        let window = PositionAndSize {
            size: self.size,
            position: Point::zero(),
        };
        hero.fully_contained_in(&window)
    }

    fn hero_size(&self) -> Size {
        let (_, ref hero) = self.hero;
        hero.size
    }
}
