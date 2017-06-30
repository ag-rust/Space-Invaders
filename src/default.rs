pub trait Default {
    fn default() -> Self;
}

pub fn default<T>() -> T
    where T: Default
{
    Default::default()
}
