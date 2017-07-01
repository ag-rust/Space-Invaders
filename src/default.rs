use std::collections::HashMap;
use std::hash::Hash;

pub trait Default {
    fn default() -> Self;
}

pub fn default<T>() -> T
    where T: Default
{
    Default::default()
}

impl<K, V> Default for HashMap<K, V>
    where K: Hash + Eq
{
    fn default() -> Self {
        HashMap::new()
    }
}
