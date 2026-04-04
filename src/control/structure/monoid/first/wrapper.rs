use crate::base::hkt::TypeConstructor1;
use crate::base::value::{Concurrent, SimpleValue, Value};
use crate::data::maybe::Maybe;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct First<T>(pub Maybe<T>);

impl<T> First<T> {
    pub fn get(s: Self) -> Maybe<T> {
        s.0
    }
}

impl<T> SimpleValue for First<T> where T: Value {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FirstInstance;

impl TypeConstructor1 for FirstInstance {
    type Type<A1>
        = First<A1>
    where
        A1: Concurrent;
}
