use default::*;
use std::ops::Sub;

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub height: u32,
    pub width: u32,
}

impl Size {
    pub fn zero() -> Size {
        Size { height: 0, width: 0 }
    }
}

impl Sub for Size {
    type Output = Size;
    fn sub(self, other: Size) -> Size {
        Size { width: self.width - other.width, height: self.height - other.height }
    }
}

impl Default for Size {
    fn default() -> Size {
        Size::zero()
    }
}
