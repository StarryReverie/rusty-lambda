use crate::base::value::{SimpleValue, Value};
use crate::control::context::ContextConstructor;
use crate::data::maybe::{Maybe, MaybeInstance};

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

impl ContextConstructor for LastInstance {
    type Type<A>
        = Last<A>
    where
        A: Value;
}

crate::derive_functor_for_nested_functor!(LastInstance, Last, MaybeInstance);
crate::derive_foldable_for_nested_foldable!(LastInstance, Last, MaybeInstance);
crate::derive_traversable_for_nested_traversable!(LastInstance, Last, MaybeInstance);
crate::derive_applicative_for_nested_applicative!(LastInstance, Last, MaybeInstance);
crate::derive_alternative_for_nested_alternative!(LastInstance, Last, MaybeInstance);
crate::derive_monad_for_nested_monad!(LastInstance, Last, MaybeInstance);
