use crate::base::function::{WrappedFn, constv};
use crate::base::value::Value;
use crate::control::context::alternative::{Alternative, AlternativeExt};
use crate::control::context::monad::Monad;
use crate::control::transformer::state::{StackedStateTInstance, StateT};

impl<S, M> Alternative for StackedStateTInstance<S, M>
where
    S: Value,
    M: Monad + Alternative,
{
    fn fallback<A>() -> Self::Type<A>
    where
        A: Value,
    {
        StateT::new(constv(M::fallback()))
    }

    fn alt<A>(one: Self::Type<A>, another: Self::Type<A>) -> Self::Type<A>
    where
        A: Value,
    {
        StateT::new(WrappedFn::from(move |state: S| {
            M::alt(
                StateT::run_tr(&one, state.clone()),
                StateT::run_tr(&another, state),
            )
        }))
    }
}

impl<S, M, A> AlternativeExt for StateT<S, M, A>
where
    S: Value,
    M: Monad + Alternative,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedStateTInstance<S, M>;
}
