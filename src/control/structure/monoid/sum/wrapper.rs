use crate::base::numeric::Additive;
use crate::base::value::{SimpleValue, Value};
use crate::control::structure::monoid::Monoid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sum<T>(pub T);

impl<T> SimpleValue for Sum<T> where T: Value {}

impl<T> Default for Sum<T>
where
    T: Additive,
{
    fn default() -> Self {
        Self::empty()
    }
}
