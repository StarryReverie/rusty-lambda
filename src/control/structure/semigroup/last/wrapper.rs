use crate::base::value::{SimpleValue, StaticConcurrent, Value};
use crate::control::context::ContextConstructor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Last<T>(pub T);

impl<T> Last<T> {
    pub fn get(s: Self) -> T {
        s.0
    }
}

impl<T> SimpleValue for Last<T> where T: Value {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LastInstance;

impl ContextConstructor for LastInstance {
    type Type<A1>
        = Last<A1>
    where
        A1: StaticConcurrent;
}

crate::derive_functor_for_wrapper!(LastInstance, Last);
crate::derive_applicative_for_wrapper!(LastInstance, Last);
crate::derive_monad_for_wrapper!(LastInstance, Last);
