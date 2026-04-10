use crate::base::function::{ConcurrentFn, WrappedFn};
use crate::base::value::Value;
use crate::control::transformer::MonadTrans;
use crate::control::transformer::reader::{ReaderT, ReaderTInstance, StackedReaderTInstance};
use crate::control::transformer::state::MonadState;
use crate::control::transformer::writer::MonadWriter;

impl<R, M> MonadState for StackedReaderTInstance<R, M>
where
    R: Value,
    M: MonadState,
{
    type State = M::State;

    fn state<A, G>(run: G) -> Self::Type<A>
    where
        A: Value,
        G: Into<WrappedFn<Self::State, (A, Self::State)>>,
    {
        ReaderTInstance::lift(M::state(run))
    }
}

impl<R, M> MonadWriter for StackedReaderTInstance<R, M>
where
    R: Value,
    M: MonadWriter,
{
    type Log = M::Log;

    fn writer<A>(entries: (A, Self::Log)) -> Self::Type<A>
    where
        A: Value,
    {
        ReaderTInstance::lift(M::writer(entries))
    }

    fn listen<A>(context: Self::Type<A>) -> Self::Type<(A, Self::Log)>
    where
        A: Value,
    {
        ReaderT::new(WrappedFn::from(move |env: R| {
            M::listen(ReaderT::run_tr(&context, env))
        }))
    }

    fn pass<A, G>(context: Self::Type<(A, G)>) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Log, Output = Self::Log>>,
    {
        ReaderT::new(WrappedFn::from(move |env: R| {
            M::pass(ReaderT::run_tr(&context, env))
        }))
    }
}
