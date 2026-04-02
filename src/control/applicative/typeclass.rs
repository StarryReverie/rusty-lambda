use std::borrow::Borrow;

use crate::base::value::{StaticConcurrent, Value};
use crate::control::functor::Functor;

pub trait Applicative: Functor {
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value;

    fn apply<A, B, G, GI>(g: Self::Type<G>, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: Borrow<GI> + Value,
        GI: Fn(A) -> B + StaticConcurrent;

    fn achain<A>(x: Self::Type<A>) -> ApplicativeChain<Self, A>
    where
        Self: Sized,
        A: Value,
    {
        ApplicativeChain::new(x)
    }

    fn apure<A>(x: A) -> ApplicativeChain<Self, A>
    where
        Self: Sized,
        A: Value,
    {
        Self::achain(Self::pure(x))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ApplicativeChain<I, A>
where
    I: Applicative,
    A: StaticConcurrent,
{
    value: I::Type<A>,
}

impl<I, A> ApplicativeChain<I, A>
where
    I: Applicative,
    A: StaticConcurrent,
{
    fn new(value: I::Type<A>) -> Self {
        Self { value }
    }

    pub fn eval(self) -> I::Type<A> {
        self.value
    }
}

impl<I, G> ApplicativeChain<I, G>
where
    I: Applicative,
    G: Value,
{
    pub fn apply<A, B, GI>(self, x: I::Type<A>) -> ApplicativeChain<I, B>
    where
        A: Value,
        B: Value,
        G: Borrow<GI>,
        GI: Fn(A) -> B + StaticConcurrent,
    {
        ApplicativeChain::new(I::apply(self.value, x))
    }
}
