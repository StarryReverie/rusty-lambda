use crate::base::function::{ConcurrentFn, constv};
use crate::base::value::{Concurrent, Value};
use crate::control::context::applicative::Applicative;

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

    fn then<A, B>(x: Self::Type<A>, y: Self::Type<B>) -> Self::Type<B>
    where
        Self: Sized,
        A: Value,
        B: Value,
        Self::Type<B>: Value,
    {
        Self::bind(x, constv(y))
    }

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

    pub fn then<B>(self, y: I::Type<B>) -> MonadChain<I, B>
    where
        B: Value,
        I::Type<B>: Value,
    {
        MonadChain::new(I::then(self.value, y))
    }
}

#[macro_export]
macro_rules! derive_monad_for_wrapper {
    ($instance:ty) => {
        impl $crate::control::context::monad::Monad for $instance {
            fn bind<A, B, G>(x: Self::Type<A>, g: G) -> Self::Type<B>
            where
                A: $crate::base::value::Value,
                B: $crate::base::value::Value,
                G: for<'a> $crate::base::value::Value<
                        View<'a>: $crate::base::function::ConcurrentFn<A, Output = Self::Type<B>>,
                    >,
            {
                $crate::base::function::ConcurrentFn::call(&g.view(), x.0)
            }
        }
    };
}
