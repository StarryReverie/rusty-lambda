use crate::base::function::WrappedFn;
use crate::base::value::Value;
use crate::control::transformer::MonadTrans;
use crate::control::transformer::maybe::{MaybeTInstance, StackedMaybeTInstance};
use crate::control::transformer::state::MonadState;

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
