use crate::base::function::{ConcurrentFn, WrappedFn};
use crate::base::value::Value;
use crate::control::transformer::MonadTrans;
use crate::control::transformer::maybe::{MaybeT, MaybeTInstance, StackedMaybeTInstance};
use crate::control::transformer::reader::MonadReader;
use crate::control::transformer::state::MonadState;

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
