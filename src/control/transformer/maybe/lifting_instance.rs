use crate::base::function::{ConcurrentFn, WrappedFn, id};
use crate::base::value::Value;
use crate::control::transformer::MonadTrans;
use crate::control::transformer::except::MonadExcept;
use crate::control::transformer::maybe::{MaybeT, MaybeTInstance, StackedMaybeTInstance};
use crate::control::transformer::reader::MonadReader;
use crate::control::transformer::state::MonadState;
use crate::control::transformer::writer::MonadWriter;
use crate::data::maybe::Maybe;

impl<M> MonadExcept for StackedMaybeTInstance<M>
where
    M: MonadExcept,
{
    type Error = M::Error;

    fn throw_error<A>(error: Self::Error) -> Self::Type<A>
    where
        A: Value,
    {
        MaybeTInstance::lift(M::throw_error(error))
    }

    fn catch_error<A, G>(fallible: Self::Type<A>, handler: G) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Error, Output = Self::Type<A>>>,
    {
        MaybeT::new(M::catch_error(
            MaybeT::run_tr(fallible),
            WrappedFn::from(move |error| MaybeT::run_tr(handler.view().call(error))),
        ))
    }
}

impl<M> MonadReader for StackedMaybeTInstance<M>
where
    M: MonadReader,
{
    type Environment = M::Environment;

    fn ask() -> Self::Type<Self::Environment> {
        MaybeTInstance::lift(M::ask())
    }

    fn local<A, G>(localize: G, context: Self::Type<A>) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Environment, Output = Self::Environment>>,
    {
        MaybeT::new(M::local(localize, MaybeT::run_tr(context)))
    }
}

impl<M> MonadState for StackedMaybeTInstance<M>
where
    M: MonadState,
{
    type State = M::State;

    fn state<A, G>(run: G) -> Self::Type<A>
    where
        A: Value,
        G: Into<WrappedFn<Self::State, (A, Self::State)>>,
    {
        MaybeTInstance::lift(M::state(run))
    }
}

impl<M> MonadWriter for StackedMaybeTInstance<M>
where
    M: MonadWriter,
{
    type Log = M::Log;

    fn writer<A>(entries: (A, Self::Log)) -> Self::Type<A>
    where
        A: Value,
    {
        MaybeTInstance::lift(M::writer(entries))
    }

    fn listen<A>(context: Self::Type<A>) -> Self::Type<(A, Self::Log)>
    where
        A: Value,
    {
        MaybeT::new(M::fmap(
            &(|(mx, log)| match mx {
                Maybe::Just(x) => Maybe::Just((x, log)),
                Maybe::Nothing => Maybe::Nothing,
            }),
            M::listen(MaybeT::run_tr(context)),
        ))
    }

    fn pass<A, G>(context: Self::Type<(A, G)>) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Log, Output = Self::Log>>,
    {
        MaybeT::new(M::pass(M::fmap(
            &(|inner: Maybe<(A, G)>| match inner {
                Maybe::Just((x, map)) => {
                    let map = WrappedFn::from(move |x| map.view().call(x));
                    (Maybe::Just(x), map)
                }
                Maybe::Nothing => (Maybe::Nothing, id()),
            }),
            MaybeT::run_tr(context),
        )))
    }
}
