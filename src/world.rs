extern crate rand;

use std::collections::HashMap;
use color::Color;
use size::Size;
use entity::*;
use max_min::*;
use std::time::Duration;
use point::*;
use config::*;
use score::*;
use rand::*;
use game_state::*;
use grid_distribution::*;
use direction::*;
use default;
use std::time::{Instant};
use rand::distributions::{IndependentSample, Range};

pub struct World {
    pub hero: (Point, Entity),
    pub enemies: HashMap<Point, Entity>,
    pub projectiles: HashMap<Point, Entity>,
    pub start_time: Instant,
    pub last_enemy_vertical_move: Instant,
    pub last_enemy_horizontal_move: Instant,
    pub last_shot: Instant,
    pub rng: ThreadRng,
    pub config: Config,
    pub score: Score,
    pub game_state: GameState,
    pub enemy_horizontal_direction: Direction,
    world_space: PositionAndSize,
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
            enemies: default::default(),
            projectiles: default::default(),
            start_time: Instant::now(),
            last_enemy_horizontal_move: Instant::now(),
            last_enemy_vertical_move: Instant::now(),
            last_shot: Instant::now(),
            rng: rand::thread_rng(),
            world_space: PositionAndSize {
                size: config.world_size.clone(),
                position: Point::zero(),
            },
            config: config,
            score: default::default(),
            game_state: default::default(),
            enemy_horizontal_direction: default::default(),
        };

        world
    }

    pub fn start(&mut self) {
        self.hero.0 = self.config.hero_starting_position;
        self.enemies = default::default();
        self.score = default::default();

        self.populate_with_enemies();

        self.game_state = GameState::Playing;
    }

    pub fn add_enemy_at(&mut self, position: Point, enemy: Entity) {
        self.enemies.insert(position, enemy);
    }

    pub fn populate_with_enemies(&mut self) {
        let remaining_space = self.config.world_size.width - self.config.enemy_grid_size.width;
        let padding_on_sides_of_grid = (remaining_space as f64 / 2.0) as i32;

        let distribution = GridDistribution {
            available_space: self.config.enemy_grid_size,
            entity_size: self.config.enemy_size,
            horizontal_padding: 10,
            vertical_padding: 10,
        }.distribute();

        for point in distribution {
            let enemy = Entity {
                size: Size {
                    height: self.config.enemy_size.height,
                    width: self.config.enemy_size.width
                },
                color: self.config.enemy_color,
            };

            self.add_enemy_at(
                point + Point { x: padding_on_sides_of_grid, y: 20 },
                enemy
                );
        }
    }
}

// Moving the hero
impl World {
    pub fn move_up(&mut self) {
        if !self.config.vertical_movement_allowed { return }
        let (current_pos, _) = self.hero;
        if current_pos.y <= 0 { return }
        let new_pos = Point { y: current_pos.y - self.config.hero_speed as i32, .. current_pos };
        self.hero.0 = new_pos;
        self.check_if_still_alive();
    }

    pub fn move_down(&mut self) {
        if !self.config.vertical_movement_allowed { return }
        let (current_pos, _) = self.hero;
        if current_pos.y + self.hero.1.size.height as i32 >= self.config.world_size.height as i32 { return }
        let new_pos = Point { y: current_pos.y + self.config.hero_speed as i32, .. current_pos };
        self.hero.0 = new_pos;
        self.check_if_still_alive();
    }

    pub fn move_left(&mut self) {
        if !self.config.horizontal_movement_allowed { return }
        let (current_pos, _) = self.hero;
        if current_pos.x <= 0 { return }
        let new_pos = Point { x: current_pos.x - self.config.hero_speed as i32 , .. current_pos };
        self.hero.0 = new_pos;
        self.check_if_still_alive();
    }

    pub fn move_right(&mut self) {
        if !self.config.horizontal_movement_allowed { return }
        let (current_pos, _) = self.hero;
        if current_pos.x + self.hero.1.size.width as i32 >= self.config.world_size.width as i32 { return }
        let new_pos = Point { x: current_pos.x + self.config.hero_speed as i32 , .. current_pos };
        self.hero.0 = new_pos;
        self.check_if_still_alive();
    }

    pub fn check_if_still_alive(&mut self) {
        if self.dead() { return };

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
                self.game_state = GameState::Dead;
                return;
            }
        }
    }

    pub fn dead(&self) -> bool {
        match self.game_state {
            GameState::Dead => true,
            _ => false,
        }
    }
    pub fn alive(&self) -> bool { !self.dead() }

    fn time_to_move_enemies_horizontally(&self) -> bool {
        self.last_enemy_horizontal_move.elapsed() >= self.config.move_enemies_horizontally_every
    }

    fn time_to_move_enemies_vertically(&self) -> bool {
        self.last_enemy_vertical_move.elapsed() >= self.config.move_enemies_vertically_every
    }

    pub fn move_enemies(&mut self) {
        self.move_enemies_vertically();
        self.move_enemies_horizontally();
    }

    fn move_enemies_vertically(&mut self) {
        if !self.time_to_move_enemies_vertically() { return }

        let mut new_positions: HashMap<Point, Entity> = default::default();

        for (point, enemy) in &self.enemies {
            let delta = Point {
                x: 0,
                y: self.config.enemy_vertical_speed as i32,
            };

            new_positions.insert(
                point.clone() + delta,
                enemy.clone(),
                );
        }

        self.enemies = new_positions;
        self.last_enemy_vertical_move = Instant::now()
    }

    fn move_enemies_horizontally(&mut self) {
        if !self.time_to_move_enemies_horizontally() { return }

        let mut new_positions: HashMap<Point, Entity> = default::default();

        for (point, enemy) in &self.enemies {
            let delta = Point {
                x: match self.enemy_horizontal_direction {
                    Direction::Right => {
                        self.config.enemy_horizontal_speed as i32
                    },
                    Direction::Left => {
                        -1 * self.config.enemy_horizontal_speed as i32
                    },
                },
                y: 0,
            };

            new_positions.insert(
                point.clone() + delta,
                enemy.clone(),
                );
        }

        self.enemies = new_positions;
        self.last_enemy_horizontal_move = Instant::now()
    }

    pub fn check_for_out_of_bounds_enemies(&mut self) {
        let any_out_of_bounds = self.enemies.iter().any(|(p, e)| {
            let pos_size = PositionAndSize {
                position: p.clone(),
                size: e.size,
            };
            !pos_size.fully_contained_in(&self.world_space)
        });

        if any_out_of_bounds {
            self.enemy_horizontal_direction = self.enemy_horizontal_direction.flip();
            self.last_enemy_horizontal_move = Instant::now() - self.config.move_enemies_horizontally_every;
        }
    }

    pub fn check_if_enemy_at_bottom(&mut self) {
        let any_reached_bottom = self.enemies.iter().any(|(p, e)| {
            p.y + e.size.height as i32 >= self.config.world_size.height as i32
        });

        if any_reached_bottom {
            self.game_state = GameState::Dead;
        }
    }
}

// Shooting projectiles
impl World {
    pub fn shoot(&mut self) {
        if !self.allowed_to_shoot() { return }

        let projectile = Entity {
            size: self.config.projectile_size,
            color: self.config.projectile_color,
        };

        self.projectiles.insert(
            Point {
                x: self.hero.0.x + (self.hero.1.size.width as f64 / 2.0) as i32,
                y: self.hero.0.y,
            },
            projectile,
            );

        self.last_shot = Instant::now();
    }

    fn allowed_to_shoot(&self) -> bool {
        self.last_shot.elapsed() >= self.config.time_between_shots
    }

    pub fn move_projectiles(&mut self) {
        let mut new_positions: HashMap<Point, Entity> = default::default();

        for (point, projectile) in &self.projectiles {
            new_positions.insert(
                point.clone() + Point { x: 0, y: -1 * self.config.projectile_speed as i32 },
                projectile.clone()
                );
        }

        self.projectiles = new_positions;
    }

    pub fn check_for_projectile_enemy_collisions(&mut self) {
        let mut enemies_to_remove: Vec<Point> = Vec::new();
        let mut projectiles_to_remove: Vec<Point> = Vec::new();

        for (projectile_point, projectile) in &self.projectiles {
            if projectile_point.y <= 0 {
                projectiles_to_remove.push(projectile_point.clone());
                continue;
            }

            for (enemy_point, enemy) in &self.enemies {
                let projectile_space = PositionAndSize {
                    size: projectile.size,
                    position: projectile_point.clone(),
                };
                let enemy_space = PositionAndSize {
                    size: enemy.size,
                    position: enemy_point.clone(),
                };
                if projectile_space.collides_with(&enemy_space) {
                    enemies_to_remove.push(enemy_point.clone());
                    projectiles_to_remove.push(projectile_point.clone());
                }
            }
        }

        for point in enemies_to_remove {
            self.enemies.remove(&point);
            self.killed_enemy_at(&point);
        }
        for point in projectiles_to_remove {
            self.projectiles.remove(&point);
        }
    }

    fn killed_enemy_at(&mut self, point: &Point) {
        self.score = self.score + self.config.points_for_killing_enemy;
    }

    pub fn check_if_won(&mut self) {
        if self.enemies.is_empty() {
            self.game_state = GameState::Won;
        }
    }
}
