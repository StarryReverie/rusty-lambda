use crate::base::function::{ConcurrentFn, WrappedFn, id};
use crate::base::value::Value;
use crate::control::transformer::MonadTrans;
use crate::control::transformer::except::{ExceptT, ExceptTInstance, StackedExceptTInstance};
use crate::control::transformer::reader::MonadReader;
use crate::control::transformer::state::MonadState;
use crate::control::transformer::writer::MonadWriter;
use crate::data::either::Either;

impl<E, M> MonadReader for StackedExceptTInstance<E, M>
where
    E: Value,
    M: MonadReader,
{
    type Environment = M::Environment;

    fn ask() -> Self::Type<Self::Environment> {
        ExceptTInstance::lift(M::ask())
    }

    fn local<A, G>(localize: G, context: Self::Type<A>) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Environment, Output = Self::Environment>>,
    {
        ExceptT::new(M::local(localize, ExceptT::run_tr(context)))
    }
}

impl<E, M> MonadState for StackedExceptTInstance<E, M>
where
    E: Value,
    M: MonadState,
{
    type State = M::State;

    fn state<A, G>(run: G) -> Self::Type<A>
    where
        A: Value,
        G: Into<WrappedFn<Self::State, (A, Self::State)>>,
    {
        ExceptTInstance::lift(M::state(run))
    }
}

impl<E, M> MonadWriter for StackedExceptTInstance<E, M>
where
    E: Value,
    M: MonadWriter,
{
    type Log = M::Log;

    fn writer<A>(entries: (A, Self::Log)) -> Self::Type<A>
    where
        A: Value,
    {
        ExceptTInstance::lift(M::writer(entries))
    }

    fn listen<A>(context: Self::Type<A>) -> Self::Type<(A, Self::Log)>
    where
        A: Value,
    {
        ExceptT::new(M::fmap(
            &(|(mx, log)| match mx {
                Either::Left(e) => Either::Left(e),
                Either::Right(x) => Either::Right((x, log)),
            }),
            M::listen(ExceptT::run_tr(context)),
        ))
    }

    fn pass<A, G>(context: Self::Type<(A, G)>) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Log, Output = Self::Log>>,
    {
        ExceptT::new(M::pass(M::fmap(
            &(|inner: Either<E, (A, G)>| match inner {
                Either::Left(e) => (Either::Left(e), id()),
                Either::Right((x, map)) => {
                    let map = WrappedFn::from(move |x| map.view().call(x));
                    (Either::Right(x), map)
                }
            }),
            ExceptT::run_tr(context),
        )))
    }
}
