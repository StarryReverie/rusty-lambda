use crate::base::computation::Thunk;
use crate::base::function::{ConcurrentFn, WrappedFn, id};
use crate::base::value::Value;
use crate::control::transformer::MonadTrans;
use crate::control::transformer::except::MonadExcept;
use crate::control::transformer::logic::{
    LogicT, LogicTInstance, LogicTStep, StackedLogicTInstance,
};
use crate::control::transformer::reader::MonadReader;
use crate::control::transformer::state::MonadState;
use crate::control::transformer::writer::MonadWriter;
use crate::data::maybe::Maybe;

impl<M> MonadExcept for StackedLogicTInstance<M>
where
    M: MonadExcept,
{
    type Error = M::Error;

    fn throw_error<A>(error: Self::Error) -> Self::Type<A>
    where
        A: Value,
    {
        LogicTInstance::lift(M::throw_error(error))
    }

    fn catch_error<A, G>(fallible: Self::Type<A>, handler: G) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Error, Output = Self::Type<A>>>,
    {
        LogicT::new(Thunk::lazy(move || {
            M::catch_error(
                fallible.decompose(),
                WrappedFn::from(move |error| handler.view().call(error).decompose()),
            )
        }))
    }
}

impl<M> MonadReader for StackedLogicTInstance<M>
where
    M: MonadReader,
{
    type Environment = M::Environment;

    fn ask() -> Self::Type<Self::Environment> {
        LogicTInstance::lift(M::ask())
    }

    fn local<A, G>(localize: G, context: Self::Type<A>) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Environment, Output = Self::Environment>>,
    {
        LogicT::new(Thunk::lazy(move || M::local(localize, context.decompose())))
    }
}

impl<M> MonadState for StackedLogicTInstance<M>
where
    M: MonadState,
{
    type State = M::State;

    fn state<A, G>(run: G) -> Self::Type<A>
    where
        A: Value,
        G: Into<WrappedFn<Self::State, (A, Self::State)>>,
    {
        LogicTInstance::lift(M::state(run))
    }
}

impl<M> MonadWriter for StackedLogicTInstance<M>
where
    M: MonadWriter,
{
    type Log = M::Log;

    fn writer<A>(entries: (A, Self::Log)) -> Self::Type<A>
    where
        A: Value,
    {
        LogicTInstance::lift(M::writer(entries))
    }

    fn listen<A>(context: Self::Type<A>) -> Self::Type<(A, Self::Log)>
    where
        A: Value,
    {
        LogicT::new(Thunk::lazy(move || {
            M::fmap(
                &(|(xs, log)| match xs {
                    Maybe::Nothing => Maybe::Nothing,
                    Maybe::Just((x, xs)) => Maybe::Just(((x, log), Self::listen(xs))),
                }),
                M::listen(context.decompose()),
            )
        }))
    }

    fn pass<A, G>(context: Self::Type<(A, G)>) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Log, Output = Self::Log>>,
    {
        LogicT::new(Thunk::lazy(move || {
            M::pass(M::fmap(
                &(|xs: LogicTStep<M, (A, G)>| match xs {
                    Maybe::Nothing => (Maybe::Nothing, id()),
                    Maybe::Just(((x, map), xs)) => {
                        let map = WrappedFn::from(move |log| map.view().call(log));
                        (Maybe::Just((x, Self::pass(xs))), map)
                    }
                }),
                context.decompose(),
            ))
        }))
    }
}
