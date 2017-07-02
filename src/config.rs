use size::*;
use point::*;
use std::time::Duration;
use color::*;
use utils::*;
use score::*;

pub struct Config {
    pub enemy_size: Size,
    pub enemy_color: Color,
    pub enemy_horizontal_speed: u32,
    pub enemy_vertical_speed: u32,
    pub enemy_grid_size: Size,
    pub enemy_horizontal_padding: u32,
    pub enemy_vertical_padding: u32,
    pub move_enemies_horizontally_every: Duration,
    pub enemy_shoots_every: Duration,
    pub move_enemies_vertically_every: Duration,
    pub time_between_shots: Duration,
    pub hero_size: Size,
    pub hero_speed: u32,
    pub hero_color: Color,
    pub hero_starting_position: Point,
    pub number_of_enemies: u32,
    pub starting_safe_zone_around_hero_size: u32,
    pub world_size: Size,
    pub vertical_movement_allowed: bool,
    pub horizontal_movement_allowed: bool,
    pub world_background_color: Color,
    pub projectile_size: Size,
    pub projectile_color: Color,
    pub projectile_speed: u32,
    pub points_for_killing_enemy: Score,
}

impl Config {
    pub fn default() -> Config {
        let mut c = Config {
            enemy_size: Size { height: 20, width: 20 },
            enemy_color: Color::red(),
            enemy_horizontal_speed: 1,
            enemy_vertical_speed: 1,
            enemy_horizontal_padding: 15,
            enemy_vertical_padding: 15,
            enemy_shoots_every: Duration::from_millis(1000),
            move_enemies_horizontally_every: Duration::from_millis(10),
            move_enemies_vertically_every: Duration::from_millis(300),
            time_between_shots: Duration::from_millis(250),
            hero_size: Size { height: 20, width: 20 },
            hero_starting_position: Point { x: 0, y: 0 },
            hero_speed: 10,
            hero_color: Color::green(),
            number_of_enemies: 10,
            starting_safe_zone_around_hero_size: 20,
            world_size: Size { height: 320, width: 570 },
            enemy_grid_size: Size { height: 100, width: 300 },
            vertical_movement_allowed: false,
            horizontal_movement_allowed: true,
            world_background_color: Color::black(),
            projectile_size: Size { height: 6, width: 3 },
            projectile_color: Color::white(),
            projectile_speed: 2,
            points_for_killing_enemy: Score { score: 1 },
        };

        c.hero_starting_position.x = 0;
        c.hero_starting_position.y = 75.percent_of(c.world_size.height);

        c
    }
}
