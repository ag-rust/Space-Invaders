use color::Color;
use size::Size;
use point::*;

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub size: Size,
    pub color: Color,
}

pub struct PositionAndSize {
    pub size: Size,
    pub position: Point,
}

impl PositionAndSize {
    pub fn collides_with(&self, other: &PositionAndSize) -> bool {
        self.corners().into_iter().any(|point| {
          point.overlaps_with(&other.position, &other.size)
        })
    }

    pub fn fully_contained_in(&self, other: &PositionAndSize) -> bool {
        self.corners().into_iter().all(|point| {
          point.overlaps_with(&other.position, &other.size)
        })
    }

    pub fn corners(&self) -> Vec<Point> {
      let mut corners: Vec<Point> = vec![];
      corners.push(self.left_top());
      corners.push(self.right_top());
      corners.push(self.left_bottom());
      corners.push(self.right_bottom());
      corners
    }

    fn left_top(&self) -> Point {
      self.position
    }

    fn right_top(&self) -> Point {
      Point {
        x: self.position.x + self.size.width as i32,
        y: self.position.y,
      }
    }

    fn left_bottom(&self) -> Point {
      Point {
        x: self.position.x,
        y: self.position.y + self.size.height as i32,
      }
    }

    fn right_bottom(&self) -> Point {
      Point {
        x: self.position.x + self.size.width as i32,
        y: self.position.y + self.size.height as i32,
      }
    }
}
