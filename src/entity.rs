use color::Color;
use size::Size;
use point::*;

#[derive(Debug)]
pub struct Entity {
    pub size: Size,
    pub color: Color,
}

pub struct PositionAndSize {
    pub size: Size,
    pub position: Point,
}

impl PositionAndSize {
    pub fn hit_test(&self, other: &PositionAndSize) -> bool {
      for corner in self.corners() {
          if corner.overlaps_with(&other.position, &other.size) {
              return true
          }
      }

      false
    }

    fn corners(&self) -> Vec<Point> {
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
        x: self.position.x + self.size.width,
        y: self.position.y,
      }
    }

    fn left_bottom(&self) -> Point {
      Point {
        x: self.position.x,
        y: self.position.y + self.size.height,
      }
    }

    fn right_bottom(&self) -> Point {
      Point {
        x: self.position.x + self.size.width,
        y: self.position.y + self.size.height,
      }
    }
}
