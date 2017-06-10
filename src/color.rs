#[derive(Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl Color {
    pub fn black() -> Color {
        Color { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }
    }

    pub fn red() -> Color {
        Color { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }
    }

    pub fn green() -> Color {
        Color { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }
    }

    pub fn blue() -> Color {
        Color { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }
    }
}
