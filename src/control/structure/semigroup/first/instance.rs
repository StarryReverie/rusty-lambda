use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::context::applicative::Applicative;
use crate::control::context::monad::Monad;
use crate::control::structure::functor::Functor;
use crate::control::structure::semigroup::first::{First, FirstInstance};

impl Functor for FirstInstance {
    fn fmap<A, B, G>(g: G, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        First(g.view().call(x.0))
    }
}

impl Applicative for FirstInstance {
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        First(x)
    }

    fn apply<A, B, G>(g: Self::Type<G>, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        First(g.0.view().call(x.0))
    }
}

impl Monad for FirstInstance {
    fn bind<A, B, G>(x: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        g.view().call(x.0)
    }
}
