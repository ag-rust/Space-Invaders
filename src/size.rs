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
