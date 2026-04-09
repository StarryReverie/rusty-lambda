use crate::base::value::{SimpleValue, Value};
use crate::control::context::ContextConstructor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct First<T>(pub T);

impl<T> First<T> {
    pub fn get_first(s: Self) -> T {
        s.0
    }
}

impl<T> SimpleValue for First<T> where T: Value {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FirstInstance;

impl ContextConstructor for FirstInstance {
    type Type<A>
        = First<A>
    where
        A: Value;
}

crate::derive_functor_for_wrapper!(FirstInstance, First);
crate::derive_applicative_for_wrapper!(FirstInstance, First);
crate::derive_monad_for_wrapper!(FirstInstance, First);
