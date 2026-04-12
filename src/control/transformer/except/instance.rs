use crate::base::function::{ConcurrentFn, Curry, WrappedFn};
use crate::base::value::Value;
use crate::control::context::applicative::{Applicative, ApplicativeExt};
use crate::control::context::monad::{Monad, MonadExt};
use crate::control::structure::functor::{Functor, FunctorExt};
use crate::control::transformer::except::ExceptT;
use crate::control::transformer::except::StackedExceptTInstance;
use crate::data::either::{Either, EitherInstance};

impl<E, M> Functor for StackedExceptTInstance<E, M>
where
    E: Value,
    M: Functor,
{
    fn fmap<A, B, G>(g: G, fx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        ExceptT::new(M::fmap(
            WrappedFn::from(move |x| EitherInstance::fmap(g.clone(), x)),
            ExceptT::run_tr(fx),
        ))
    }
}

impl<E, M, A> FunctorExt for ExceptT<E, M, A>
where
    E: Value,
    M: Functor,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedExceptTInstance<E, M>;
}

impl<E, M> Applicative for StackedExceptTInstance<E, M>
where
    E: Value,
    M: Monad,
{
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        ExceptT::new(M::pure(EitherInstance::pure(x)))
    }

    fn apply<A, B, G>(mg: Self::Type<G>, mx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        ExceptT::new(M::bind(
            ExceptT::run_tr(mg),
            WrappedFn::from(move |g| match g {
                Either::Left(e) => M::pure(Either::Left(e)),
                Either::Right(g) => M::fmap(
                    WrappedFn::curry(EitherInstance::fmap)(g),
                    ExceptT::run_tr(mx.clone()),
                ),
            }),
        ))
    }
}

impl<E, M, A> ApplicativeExt for ExceptT<E, M, A>
where
    E: Value,
    M: Monad,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedExceptTInstance<E, M>;
}

impl<E, M> Monad for StackedExceptTInstance<E, M>
where
    E: Value,
    M: Monad,
{
    fn bind<A, B, G>(mx: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        ExceptT::new(M::bind(
            ExceptT::run_tr(mx),
            WrappedFn::from(move |x| match x {
                Either::Left(e) => M::pure(Either::Left(e)),
                Either::Right(x) => ExceptT::run_tr(g.view().call(x)),
            }),
        ))
    }
}

impl<E, M, A> MonadExt for ExceptT<E, M, A>
where
    E: Value,
    M: Monad,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedExceptTInstance<E, M>;
}
