pub trait PercentageCalculation<In, Out> {
    fn percent_of(self, In) -> Out;
}

impl PercentageCalculation<u32, u32> for u32 {
    fn percent_of(self, x: u32) -> u32 {
        (x as f64 * (self as f64 / 100.0)) as u32
    }
}

impl PercentageCalculation<u32, f64> for u32 {
    fn percent_of(self, x: u32) -> f64 {
        (x as f64 * (self as f64 / 100.0))
    }
}

impl PercentageCalculation<u32, i32> for i32 {
    fn percent_of(self, x: u32) -> i32 {
        (x as f64 * (self as f64 / 100.0)) as i32
    }
}
