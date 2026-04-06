use crate::base::value::{Concurrent, Value};
use crate::control::context::applicative::Applicative;

pub trait Alternative: Applicative {
    fn fallback<A>() -> Self::Type<A>
    where
        A: Concurrent;

    fn alt<A>(one: Self::Type<A>, another: Self::Type<A>) -> Self::Type<A>
    where
        A: Value;

    fn fchain<A>(one: Self::Type<A>) -> AlternativeChain<Self, A>
    where
        A: Concurrent,
        Self: Sized,
    {
        AlternativeChain(one)
    }

    fn ffallback<A>() -> AlternativeChain<Self, A>
    where
        A: Concurrent,
        Self: Sized,
    {
        AlternativeChain(Self::fallback())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AlternativeChain<I, A>(I::Type<A>)
where
    I: Alternative,
    A: Concurrent;

impl<I, A> AlternativeChain<I, A>
where
    I: Alternative,
    A: Value,
{
    pub fn alt(self, another: I::Type<A>) -> Self {
        Self(I::alt(self.0, another))
    }

    pub fn eval(self) -> I::Type<A> {
        self.0
    }
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
