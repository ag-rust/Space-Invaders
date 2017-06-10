use entity::Entity;
use color::Color;
use point::*;
use graphics::*;
use opengl_graphics::{GlGraphics};

pub trait Drawable {
    fn draw(&self, context: &Context, gl: &mut GlGraphics);
}

impl Drawable for (Point, Entity) {
    fn draw(&self, context: &Context, gl: &mut GlGraphics) {
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
