use crate::base::hkt::TypeConstructor1;
use crate::base::value::{Concurrent, SimpleValue, Value};
use crate::data::maybe::Maybe;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Last<T>(pub Maybe<T>);

impl<T> Last<T> {
    pub fn get(s: Self) -> Maybe<T> {
        s.0
    }
}

impl<T> SimpleValue for Last<T> where T: Value {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LastInstance;

impl TypeConstructor1 for LastInstance {
    type Type<A1>
        = Last<A1>
    where
        A1: Concurrent;
}
