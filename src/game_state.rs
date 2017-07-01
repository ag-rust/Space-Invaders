use default::*;

pub enum GameState {
    Intro,
    Playing,
    Dead,
    Won,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Intro
    }
}
