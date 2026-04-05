use crate::base::function::ConcurrentFn;
use crate::base::value::{Concurrent, Value};
use crate::control::structure::functor::Functor;

pub trait Applicative: Functor {
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value;

    fn apply<A, B, G>(g: Self::Type<G>, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>;

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
    A: Concurrent,
{
    value: I::Type<A>,
}

impl<I, A> ApplicativeChain<I, A>
where
    I: Applicative,
    A: Concurrent,
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
    pub fn apply<A, B>(self, x: I::Type<A>) -> ApplicativeChain<I, B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        ApplicativeChain::new(I::apply(self.value, x))
    }
}

#[macro_export]
macro_rules! derive_applicative_for_wrapper {
    ($instance:ty, $wrapper:ident) => {
        impl $crate::control::context::applicative::Applicative for $instance {
            fn pure<A>(x: A) -> Self::Type<A>
            where
                A: $crate::base::value::Value,
            {
                $wrapper(x)
            }

            fn apply<A, B, G>(g: Self::Type<G>, x: Self::Type<A>) -> Self::Type<B>
            where
                A: $crate::base::value::Value,
                B: $crate::base::value::Value,
                G: for<'a> $crate::base::value::Value<
                        View<'a>: $crate::base::function::ConcurrentFn<A, Output = B>,
                    >,
            {
                $wrapper($crate::base::function::ConcurrentFn::call(&g.0.view(), x.0))
            }
        }
    };
}
