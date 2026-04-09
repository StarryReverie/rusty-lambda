use crate::base::function::{ConcurrentFn, Curry, WrappedFn};
use crate::base::value::Value;
use crate::control::context::applicative::{Applicative, ApplicativeExt};
use crate::control::context::monad::{Monad, MonadExt};
use crate::control::structure::functor::{Functor, FunctorExt};
use crate::control::transformer::maybe::{MaybeT, StackedMaybeTInstance};
use crate::data::maybe::{Maybe, MaybeInstance};

impl<M> Functor for StackedMaybeTInstance<M>
where
    M: Functor + 'static,
{
    fn fmap<A, B, G>(g: G, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        MaybeT(M::fmap(
            WrappedFn::from(move |x| MaybeInstance::fmap(g.clone(), x)),
            x.0,
        ))
    }
}

impl<M, A> FunctorExt for MaybeT<M, A>
where
    M: Functor + 'static,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedMaybeTInstance<M>;
}

impl<M> Applicative for StackedMaybeTInstance<M>
where
    M: Applicative + 'static,
{
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        MaybeT(M::pure(MaybeInstance::pure(x)))
    }

    fn apply<A, B, G>(g: Self::Type<G>, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        MaybeT(M::apply(
            M::fmap(WrappedFn::curry(MaybeInstance::apply), g.0),
            x.0,
        ))
    }
}

impl<M, A> ApplicativeExt for MaybeT<M, A>
where
    M: Monad + 'static,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedMaybeTInstance<M>;
}

impl<M> Monad for StackedMaybeTInstance<M>
where
    M: Monad + 'static,
{
    fn bind<A, B, G>(x: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        MaybeT(M::bind(
            x.0,
            WrappedFn::from(move |x| match x {
                Maybe::Just(x) => g.view().call(x).0,
                Maybe::Nothing => M::pure(Maybe::Nothing),
            }),
        ))
    }
}

impl<M, A> MonadExt for MaybeT<M, A>
where
    M: Monad + 'static,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedMaybeTInstance<M>;
}

#[cfg(test)]
mod tests {
    use crate::control::transformer::MonadTrans;
    use crate::control::transformer::maybe::MaybeTInstance;
    use crate::data::list::{List, ListInstance};

    use super::*;

    #[test]
    fn test_monad_transformer() {
        let m = MaybeT::<ListInstance, _>(List::from(vec![
            Maybe::Just(1),
            Maybe::Just(2),
            Maybe::Just(3),
            Maybe::Just(4),
        ]));
        let m = m.bind(WrappedFn::from(|x| {
            if x % 2 == 0 {
                StackedMaybeTInstance::pure(x * 2)
            } else {
                MaybeTInstance::lift(List::singleton(-1))
            }
        }));
        eprintln!("{m:?}");
    }
}
