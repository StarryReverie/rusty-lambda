use crate::base::numeric::Multiplicative;
use crate::base::value::{SimpleValue, Value};
use crate::control::structure::monoid::Monoid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Product<T>(pub T);

impl<T> SimpleValue for Product<T> where T: Value {}

impl<T> Default for Product<T>
where
    T: Multiplicative,
{
    fn default() -> Self {
        Self::empty()
    }
}
