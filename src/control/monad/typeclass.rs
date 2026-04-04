use crate::base::function::ConcurrentFn;
use crate::base::value::{Concurrent, Value};
use crate::control::applicative::Applicative;

pub trait Monad: Applicative {
    fn ret<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        Self::pure(x)
    }

    fn bind<A, B, G>(x: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>;

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
    A: Concurrent,
{
    value: I::Type<A>,
}

impl<I, A> MonadChain<I, A>
where
    I: Monad,
    A: Concurrent,
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
    pub fn bind<B, G>(self, g: G) -> MonadChain<I, B>
    where
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = I::Type<B>>>,
    {
        MonadChain::new(I::bind(self.value, g))
    }
}
