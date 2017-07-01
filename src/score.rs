use default::*;
use std::ops::Add;
use drawing::*;
use entity::Entity;
use size::Size;
use opengl_graphics::glyph_cache::GlyphCache;
use color::Color;
use point::*;
use graphics::*;
use app::{FrameRate};
use opengl_graphics::{GlGraphics};

pub struct Score {
    pub score: u32,
}

impl Score {
    pub fn zero() -> Score {
        Score { score: 0 }
    }

    pub fn add(&self, other: Score) -> Score {
        Score { score: self.score + other.score }
    }
}

impl Default for Score {
    fn default() -> Score {
        Score::zero()
    }
}

impl Drawable for Score {
    fn draw_text(&self, window_size: Size, glyph_cache: &mut GlyphCache, context: &Context, gl: &mut GlGraphics) {
        let font_size = 10;

        let text = text::Text {
            color: Color::white().to_array(),
            font_size: font_size,
            round: true,
        };

        let fps = format!("Score: {:?}", self.score);

        let transform = context.transform.trans(
            window_size.width as f64 - 75.0,
            window_size.height as f64 - font_size as f64,
            );

        text.draw(&fps, glyph_cache, &context.draw_state, transform, gl);
    }
}
