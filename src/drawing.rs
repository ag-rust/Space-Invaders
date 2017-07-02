use entity::Entity;
use size::Size;
use opengl_graphics::glyph_cache::GlyphCache;
use color::Color;
use point::*;
use graphics::*;
use app::{FrameRate};
use opengl_graphics::{GlGraphics};

pub trait Drawable {
    fn draw_shape(&self, _window_size: Size, _context: &Context, _gl: &mut GlGraphics) {}

    fn draw_text(&self, _window_size: Size, _glyph_cache: &mut GlyphCache, _context: &Context, _gl: &mut GlGraphics) {}
}

impl Drawable for (Point, Entity) {
    fn draw_shape(&self, _window_size: Size, context: &Context, gl: &mut GlGraphics) {
        let width = self.1.size.width as f64;
        let height = self.1.size.height as f64;
        let rect = rectangle::centered([0.0, 0.0, width/2.0, height/2.0]);

        // Center of the rect is the anchor point
        let transform = context
            .transform
            .trans(
                self.0.x as f64 + width / 2.0,
                self.0.y as f64 + height / 2.0,
                );

        rectangle(
            self.1.color.to_array(),
            rect,
            transform,
            gl
            );
    }
}

impl Drawable for FrameRate {
    fn draw_text(&self, window_size: Size, glyph_cache: &mut GlyphCache, context: &Context, gl: &mut GlGraphics) {
        let font_size = 10;

        let text = text::Text {
            color: Color::white().to_array(),
            font_size: font_size,
            round: true,
        };

        let fps = format!("FPS: {}", self.fps);

        let transform = context.transform.trans(
            4.0,
            window_size.height as f64 - font_size as f64,
            );

        text.draw(&fps, glyph_cache, &context.draw_state, transform, gl);
    }
}
