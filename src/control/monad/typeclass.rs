use std::borrow::Borrow;

use crate::base::value::{StaticConcurrent, Value};
use crate::control::applicative::Applicative;

pub trait Monad: Applicative {
    fn ret<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        Self::pure(x)
    }

    fn bind<A, B, G, GI>(x: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: Borrow<GI> + Value,
        GI: Fn(A) -> Self::Type<B> + StaticConcurrent;

    fn mchain<A>(x: Self::Type<A>) -> MonadChain<Self, A>
    where
        Self: Sized,
        A: Value,
    {
        MonadChain::new(x)
    }

    fn mreturn<A>(x: A) -> MonadChain<Self, A>
    where
        Self: Sized,
        A: Value,
    {
        Self::mchain(Self::ret(x))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MonadChain<I, A>
where
    I: Monad,
    A: StaticConcurrent,
{
    value: I::Type<A>,
}

impl<I, A> MonadChain<I, A>
where
    I: Monad,
    A: StaticConcurrent,
{
    fn new(value: I::Type<A>) -> Self {
        Self { value }
    }

    pub fn eval(self) -> I::Type<A> {
        self.value
    }
}

impl<I, A> MonadChain<I, A>
where
    I: Monad,
    A: Value,
{
    pub fn bind<B, G, GI>(self, g: G) -> MonadChain<I, B>
    where
        B: Value,
        G: Borrow<GI> + Value,
        GI: Fn(A) -> I::Type<B> + StaticConcurrent,
    {
        MonadChain::new(I::bind(self.value, g))
    }
}
