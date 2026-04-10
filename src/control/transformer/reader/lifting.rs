use crate::base::function::WrappedFn;
use crate::base::value::Value;
use crate::control::transformer::MonadTrans;
use crate::control::transformer::reader::{ReaderTInstance, StackedReaderTInstance};
use crate::control::transformer::state::MonadState;

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
