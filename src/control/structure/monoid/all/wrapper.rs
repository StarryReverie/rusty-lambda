use crate::base::value::SimpleValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct All(pub bool);

impl SimpleValue for All {}

impl All {
    pub fn new() -> Self {
        Self(true)
    }
}

impl Default for All {
    fn default() -> Self {
        Self::new()
    }
}
