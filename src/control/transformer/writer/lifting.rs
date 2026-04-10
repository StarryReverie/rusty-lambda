use crate::base::function::{ConcurrentFn, WrappedFn};
use crate::base::value::Value;
use crate::control::structure::monoid::Monoid;
use crate::control::transformer::MonadTrans;
use crate::control::transformer::reader::MonadReader;
use crate::control::transformer::state::MonadState;
use crate::control::transformer::writer::{StackedWriterTInstance, WriterT, WriterTInstance};

impl<W, M> MonadReader for StackedWriterTInstance<W, M>
where
    W: Monoid + Value,
    M: MonadReader,
{
    type Environment = M::Environment;

    fn ask() -> Self::Type<Self::Environment> {
        WriterTInstance::lift(M::ask())
    }

    fn local<A, G>(localize: G, context: Self::Type<A>) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Environment, Output = Self::Environment>>,
    {
        WriterT::new(M::local(localize, WriterT::run_tr(context)))
    }
}

impl<W, M> MonadState for StackedWriterTInstance<W, M>
where
    W: Monoid + Value,
    M: MonadState,
{
    type State = M::State;

    fn state<A, G>(run: G) -> Self::Type<A>
    where
        A: Value,
        G: Into<WrappedFn<Self::State, (A, Self::State)>>,
    {
        WriterTInstance::lift(M::state(run))
    }
}
