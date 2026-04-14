use crate::base::function::{ConcurrentFn, WrappedFn};
use crate::base::value::Value;
use crate::control::transformer::MonadTrans;
use crate::control::transformer::except::MonadExcept;
use crate::control::transformer::reader::MonadReader;
use crate::control::transformer::state::{StackedStateTInstance, StateT, StateTInstance};
use crate::control::transformer::writer::MonadWriter;

impl<S, M> MonadExcept for StackedStateTInstance<S, M>
where
    S: Value,
    M: MonadExcept,
{
    type Error = M::Error;

    fn throw_error<A>(error: Self::Error) -> Self::Type<A>
    where
        A: Value,
    {
        StateTInstance::lift(M::throw_error(error))
    }

    fn catch_error<A, G>(fallible: Self::Type<A>, handler: G) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Error, Output = Self::Type<A>>>,
    {
        StateT::new(WrappedFn::from(move |state: S| {
            let handler = handler.clone();
            M::catch_error(
                StateT::run_tr(&fallible, state.clone()),
                WrappedFn::from(move |error| {
                    StateT::run_tr(handler.view().call(error), state.clone())
                }),
            )
        }))
    }
}

impl<S, M> MonadReader for StackedStateTInstance<S, M>
where
    S: Value,
    M: MonadReader,
{
    type Environment = M::Environment;

    fn ask() -> Self::Type<Self::Environment> {
        StateTInstance::lift(M::ask())
    }

    fn local<A, G>(localize: G, context: Self::Type<A>) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Environment, Output = Self::Environment>>,
    {
        StateT::new(WrappedFn::from(move |state| {
            M::local(localize.clone(), StateT::run_tr(&context, state))
        }))
    }
}

impl<S, M> MonadWriter for StackedStateTInstance<S, M>
where
    S: Value,
    M: MonadWriter,
{
    type Log = M::Log;

    fn writer<A>(entries: (A, Self::Log)) -> Self::Type<A>
    where
        A: Value,
    {
        StateTInstance::lift(M::writer(entries))
    }

    fn listen<A>(context: Self::Type<A>) -> Self::Type<(A, Self::Log)>
    where
        A: Value,
    {
        StateT::new(WrappedFn::from(move |state| {
            M::fmap(
                &(|((x, state), log)| ((x, log), state)),
                M::listen(StateT::run_tr(&context, state)),
            )
        }))
    }

    fn pass<A, G>(context: Self::Type<(A, G)>) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Log, Output = Self::Log>>,
    {
        StateT::new(WrappedFn::from(move |state| {
            M::pass(M::fmap(
                &(|((x, map), state)| ((x, state), map)),
                StateT::run_tr(&context, state),
            ))
        }))
    }
}
