use crate::base::function::{ConcurrentFn, WrappedFn};
use crate::base::value::Value;
use crate::control::context::monad::Monad;
use crate::control::transformer::except::{ExceptT, StackedExceptTInstance};
use crate::data::either::Either;

pub trait MonadExcept: Monad {
    type Error: Value;

    fn throw_error<A>(error: Self::Error) -> Self::Type<A>
    where
        A: Value;

    fn catch_error<A, G>(fallible: Self::Type<A>, handler: G) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Error, Output = Self::Type<A>>>;

    fn try_that<A>(fallible: Self::Type<A>) -> Self::Type<Either<Self::Error, A>>
    where
        A: Value,
    {
        Self::catch_error(
            Self::fmap(&Either::Right, fallible),
            &(|error| Self::pure(Either::Left(error))),
        )
    }

    fn ensure_that(condition: bool, error: Self::Error) -> Self::Type<()> {
        if condition {
            Self::pure(())
        } else {
            Self::throw_error(error)
        }
    }
}

impl<E, M> MonadExcept for StackedExceptTInstance<E, M>
where
    E: Value,
    M: Monad,
{
    type Error = E;

    fn throw_error<A>(error: Self::Error) -> Self::Type<A>
    where
        A: Value,
    {
        ExceptT::except(Either::Left(error))
    }

    fn catch_error<A, G>(fallible: Self::Type<A>, handler: G) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Error, Output = Self::Type<A>>>,
    {
        ExceptT::new(M::bind(
            ExceptT::run_tr(fallible),
            WrappedFn::from(move |res| match res {
                Either::Left(e) => ExceptT::run_tr(handler.view().call(e)),
                Either::Right(x) => M::pure(Either::Right(x)),
            }),
        ))
    }
}
