use crate::base::value::{Concurrent, Value};
use crate::control::context::applicative::Applicative;

pub trait Alternative: Applicative {
    fn fallback<A>() -> Self::Type<A>
    where
        A: Concurrent;

    fn alt<A>(one: Self::Type<A>, another: Self::Type<A>) -> Self::Type<A>
    where
        A: Value;
}

pub trait AlternativeExt {
    type Wrapped: Concurrent;
    type Instance: Alternative<Type<Self::Wrapped> = Self>;

    fn alt(self, another: Self) -> Self
    where
        Self: Sized,
        Self::Wrapped: Value,
    {
        Self::Instance::alt::<Self::Wrapped>(self, another)
    }
}
