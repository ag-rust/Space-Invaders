use default::*;

pub struct Score {
    pub score: u32,
}

impl Score {
    pub fn zero() -> Score {
        Score { score: 0 }
    }
}

impl Default for Score {
    fn default() -> Score {
        Score::zero()
    }
}
