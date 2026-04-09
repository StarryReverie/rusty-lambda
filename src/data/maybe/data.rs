use crate::base::value::{SimpleValue, StaticConcurrent, Value};
use crate::control::context::ContextConstructor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Maybe<T> {
    Just(T),
    #[default]
    Nothing,
}

impl<T> Maybe<T> {
    pub fn option(self) -> Option<T> {
        match self {
            Self::Just(x) => Some(x),
            Self::Nothing => None,
        }
    }
}

impl<T> SimpleValue for Maybe<T> where T: Value {}

impl<T, U> From<Option<U>> for Maybe<T>
where
    U: Into<T>,
{
    fn from(value: Option<U>) -> Self {
        match value {
            Some(x) => Maybe::Just(x.into()),
            None => Maybe::Nothing,
        }
    }
}

impl<T> From<Maybe<T>> for Option<T> {
    fn from(value: Maybe<T>) -> Self {
        value.option()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MaybeInstance;

impl ContextConstructor for MaybeInstance {
    type Type<A1>
        = Maybe<A1>
    where
        A1: StaticConcurrent;
}
