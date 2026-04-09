use std::marker::PhantomData;

use crate::base::function::WrappedFn;
use crate::base::value::{SimpleValue, Value};
use crate::control::context::ContextConstructor;
use crate::control::context::monad::{Monad, MonadExt};
use crate::control::structure::functor::{Functor, LhsFunctorExt};
use crate::control::transformer::state::MonadState;
use crate::control::transformer::{MonadTrans, StackedMonadTrans, TransConstructor};

pub struct StateT<S, M, A>(pub(super) WrappedFn<S, M::Type<(A, S)>>)
where
    S: Value,
    M: ContextConstructor,
    A: Value;

impl<S, M, A> StateT<S, M, A>
where
    S: Value,
    M: Monad,
    A: Value,
{
    pub fn run_tr(trans: Self, state: S) -> M::Type<(A, S)> {
        (trans.0)(state)
    }

    pub fn eval_tr(trans: Self, state: S) -> M::Type<A> {
        M::fmap(WrappedFn::from(|(x, _)| x), Self::run_tr(trans, state))
    }

    pub fn exec_tr(trans: Self, state: S) -> M::Type<S> {
        M::fmap(WrappedFn::from(|(_, s)| s), Self::run_tr(trans, state))
    }
}

impl<S, M, A, F> From<F> for StateT<S, M, A>
where
    S: Value,
    M: Monad,
    A: Value,
    F: Into<WrappedFn<S, (A, S)>>,
{
    fn from(func: F) -> Self {
        StackedStateTInstance::state(func)
    }
}

impl<S, M, A> Clone for StateT<S, M, A>
where
    S: Value,
    M: ContextConstructor,
    A: Value,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<S, M, A> SimpleValue for StateT<S, M, A>
where
    S: Value,
    M: ContextConstructor,
    A: Value,
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StateTInstance<S>(PhantomData<S>);

impl<S> TransConstructor for StateTInstance<S>
where
    S: Value,
{
    type Type<M, A>
        = StateT<S, M, A>
    where
        M: Monad,
        A: Value;

    type Stacked<M>
        = StackedStateTInstance<S, M>
    where
        M: Monad;
}

impl<S> MonadTrans for StateTInstance<S>
where
    S: Value,
{
    fn lift<A, MA>(mx: MA) -> Self::Type<MA::Instance, A>
    where
        A: Value,
        MA: MonadExt<Wrapped = A> + Value,
        Self::Stacked<MA::Instance>: Monad<Type<A> = Self::Type<MA::Instance, A>>,
    {
        StateT(WrappedFn::from(move |s: S| {
            MA::Instance::fmap(WrappedFn::from(move |x| (x, s.clone())), mx.clone())
        }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StackedStateTInstance<S, M>(PhantomData<(S, M)>);

impl<S, M> ContextConstructor for StackedStateTInstance<S, M>
where
    S: Value,
    M: ContextConstructor,
{
    type Type<A>
        = StateT<S, M, A>
    where
        A: Value;
}

impl<S, M> StackedMonadTrans for StackedStateTInstance<S, M>
where
    S: Value,
    M: Monad,
{
    type Transformer = StateTInstance<S>;
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
        StateT(WrappedFn::from(M::pure).fmap(run.into()))
    }
}
