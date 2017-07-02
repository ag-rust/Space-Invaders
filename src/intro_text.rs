use size::Size;
use utils::*;
use opengl_graphics::glyph_cache::GlyphCache;
use color::Color;
use graphics::*;
use drawing::{Drawable};
use opengl_graphics::{GlGraphics};

pub struct IntroText {
    text: String,
}

impl IntroText {
    pub fn new(text: &str) -> IntroText {
        IntroText { text: text.to_string() }
    }
}

impl Drawable for IntroText {
    fn draw_text(&self, window_size: Size, glyph_cache: &mut GlyphCache, context: &Context, gl: &mut GlGraphics) {
        let font_size = 10;

        let text = text::Text {
            color: Color::white().to_array(),
            font_size: font_size,
            round: true,
        };

        let chars = self.text.clone();

        let transform = context.transform.trans(
            10.percent_of(window_size.width),
            50.percent_of(window_size.height),
            );

        text.draw(&chars, glyph_cache, &context.draw_state, transform, gl);
    }
}
