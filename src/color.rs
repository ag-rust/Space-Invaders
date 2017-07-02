#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Color {
    pub fn to_array(&self) -> [f32; 4] {
        [
            self.red,
            self.green,
            self.blue,
            self.alpha,
        ]
    }
}

// Colors
impl Color {
    pub fn black() -> Color {
        Color { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }
    }

    pub fn white() -> Color {
        Color { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }
    }

    pub fn red() -> Color {
        Color { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }
    }

    pub fn green() -> Color {
        Color { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }
    }

    // pub fn blue() -> Color {
    //     Color { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }
    // }
}
