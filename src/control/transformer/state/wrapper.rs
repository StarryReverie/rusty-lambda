use crate::base::value::Value;
use crate::control::structure::functor::identity::IdentityInstance;
use crate::control::transformer::state::{StackedStateTInstance, StateT};

pub type State<S, A> = StateT<S, IdentityInstance, A>;
pub type StateInstance<S> = StackedStateTInstance<S, IdentityInstance>;

impl<S, A> State<S, A>
where
    S: Value,
    A: Value,
{
    pub fn run(mx: Self, state: S) -> (A, S) {
        Self::run_tr(mx, state).0
    }

    pub fn eval(mx: Self, state: S) -> A {
        Self::run(mx, state).0
    }

    pub fn exec(mx: Self, state: S) -> S {
        Self::run(mx, state).1
    }
}
