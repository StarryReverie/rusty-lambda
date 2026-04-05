use crate::base::value::SimpleValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Any(pub bool);

impl SimpleValue for Any {}

impl Any {
    pub fn new() -> Self {
        Self(false)
    }
}

impl Default for Any {
    fn default() -> Self {
        Self::new()
    }
}
