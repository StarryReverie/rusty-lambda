use crate::base::function::{ConcurrentFn, Curry, WrappedFn};
use crate::base::value::Value;
use crate::control::context::alternative::{Alternative, AlternativeExt};
use crate::control::context::applicative::{Applicative, ApplicativeExt};
use crate::control::context::monad::{Monad, MonadExt};
use crate::control::structure::functor::{Functor, FunctorExt};
use crate::control::transformer::maybe::{MaybeT, StackedMaybeTInstance};
use crate::data::maybe::{Maybe, MaybeInstance};

impl<M> Functor for StackedMaybeTInstance<M>
where
    M: Functor,
{
    fn fmap<A, B, G>(g: G, fx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        MaybeT::new(M::fmap(
            WrappedFn::from(move |x| MaybeInstance::fmap(g.clone(), x)),
            MaybeT::run_tr(fx),
        ))
    }
}

impl<M, A> FunctorExt for MaybeT<M, A>
where
    M: Functor,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedMaybeTInstance<M>;
}

impl<M> Applicative for StackedMaybeTInstance<M>
where
    M: Monad,
{
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        MaybeT::new(M::pure(MaybeInstance::pure(x)))
    }

    fn apply<A, B, G>(fg: Self::Type<G>, fx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        MaybeT::new(M::bind(
            MaybeT::run_tr(fg),
            WrappedFn::from(move |g| match g {
                Maybe::Just(g) => M::fmap(
                    WrappedFn::curry(MaybeInstance::fmap)(g),
                    MaybeT::run_tr(fx.clone()),
                ),
                Maybe::Nothing => M::pure(Maybe::Nothing),
            }),
        ))
    }
}

impl<M, A> ApplicativeExt for MaybeT<M, A>
where
    M: Monad,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedMaybeTInstance<M>;
}

impl<M> Monad for StackedMaybeTInstance<M>
where
    M: Monad,
{
    fn bind<A, B, G>(mx: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        MaybeT::new(M::bind(
            MaybeT::run_tr(mx),
            WrappedFn::from(move |x| match x {
                Maybe::Just(x) => MaybeT::run_tr(g.view().call(x)),
                Maybe::Nothing => M::pure(Maybe::Nothing),
            }),
        ))
    }
}

impl<M, A> MonadExt for MaybeT<M, A>
where
    M: Monad,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedMaybeTInstance<M>;
}

impl<M> Alternative for StackedMaybeTInstance<M>
where
    M: Monad,
{
    fn fallback<A>() -> Self::Type<A>
    where
        A: Value,
    {
        MaybeT::maybe(Maybe::Nothing)
    }

    fn alt<A>(one: Self::Type<A>, another: Self::Type<A>) -> Self::Type<A>
    where
        A: Value,
    {
        MaybeT::new(M::bind(
            MaybeT::run_tr(one),
            WrappedFn::from(move |one| match &one {
                Maybe::Just(_) => M::pure(one),
                Maybe::Nothing => MaybeT::run_tr(another.clone()),
            }),
        ))
    }
}

impl<M, A> AlternativeExt for MaybeT<M, A>
where
    M: Monad,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedMaybeTInstance<M>;
}
