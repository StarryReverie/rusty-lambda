use crate::base::hkt::TypeConstructor1;
use crate::base::value::{Concurrent, Value};

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

impl<T> Value for Maybe<T>
where
    T: Value<Unwrapped: Sized>,
{
    type Unwrapped = Maybe<T::Unwrapped>;

    type View<'a>
        = Maybe<T::View<'a>>
    where
        Self: 'a;

    fn make<U>(unwrapped: U) -> Self
    where
        U: Into<Self::Unwrapped>,
        Self::Unwrapped: Sized,
    {
        match unwrapped.into() {
            Maybe::Just(unwrapped) => Self::Just(Value::make(unwrapped)),
            Maybe::Nothing => Self::Nothing,
        }
    }

    fn view(&self) -> Self::View<'_> {
        match self {
            Self::Just(x) => Maybe::Just(x.view()),
            Self::Nothing => Maybe::Nothing,
        }
    }
}

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

impl TypeConstructor1 for MaybeInstance {
    type Type<A1>
        = Maybe<A1>
    where
        A1: Concurrent;
}
