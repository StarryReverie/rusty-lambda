use crate::base::function::{ConcurrentFn, WrappedFn};
use crate::base::value::Value;
use crate::control::context::monad::Monad;
use crate::control::structure::functor::LhsFunctorExt;
use crate::control::transformer::state::{StackedStateTInstance, StateT};

pub trait MonadState: Monad {
    type State: Value;

    fn state<A, G>(run: G) -> Self::Type<A>
    where
        A: Value,
        G: Into<WrappedFn<Self::State, (A, Self::State)>>;

    fn get() -> Self::Type<Self::State> {
        Self::state(|s: Self::State| (s.clone(), s))
    }

    fn gets<A, G>(map: G) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::State, Output = A>>,
    {
        Self::state(move |s: Self::State| (map.view().call(s.clone()), s))
    }

    fn put(state: Self::State) -> Self::Type<()> {
        Self::state(move |_| ((), state.clone()))
    }

    fn modify<G>(map: G) -> Self::Type<()>
    where
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::State, Output = Self::State>>,
    {
        Self::state(move |s| ((), map.view().call(s)))
    }
}

impl<S, M> MonadState for StackedStateTInstance<S, M>
where
    S: Value,
    M: Monad,
{
    type State = S;

    fn state<A, G>(run: G) -> Self::Type<A>
    where
        A: Value,
        G: Into<WrappedFn<Self::State, (A, Self::State)>>,
    {
        StateT::new((M::pure).fmap(run.into()))
    }
}
