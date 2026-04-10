use crate::base::function::{ConcurrentFn, WrappedFn};
use crate::base::value::Value;
use crate::control::transformer::MonadTrans;
use crate::control::transformer::reader::MonadReader;
use crate::control::transformer::state::{StackedStateTInstance, StateT, StateTInstance};

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
