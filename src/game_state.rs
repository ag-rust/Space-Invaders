use default::*;

pub enum GameState {
    Intro,
    Playing,
    Dead,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Intro
    }
}
