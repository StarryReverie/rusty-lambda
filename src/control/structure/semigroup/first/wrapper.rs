use crate::base::hkt::TypeConstructor1;
use crate::base::value::{SimpleValue, StaticConcurrent, Value};

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

impl TypeConstructor1 for FirstInstance {
    type Type<A1>
        = First<A1>
    where
        A1: StaticConcurrent;
}

crate::derive_functor_for_wrapper!(FirstInstance, First);
crate::derive_applicative_for_wrapper!(FirstInstance, First);
crate::derive_monad_for_wrapper!(FirstInstance, First);
