pub trait MaxMin<T> where T: PartialOrd + Copy {
    fn max(&self) -> T;
    fn min(&self) -> T;
}

impl<T> MaxMin<T> for (T, T) where T: PartialOrd + Copy {
    fn max(&self) -> T {
        if self.0 > self.1 { self.0 } else { self.1 }
    }

    fn min(&self) -> T {
        if self.0 < self.1 { self.0 } else { self.1 }
    }
}
