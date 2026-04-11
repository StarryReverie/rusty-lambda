use std::borrow::Borrow;
use std::marker::PhantomData;

use crate::base::function::WrappedFn;
use crate::base::value::{SimpleValue, Value};
use crate::control::context::ContextConstructor;
use crate::control::context::monad::Monad;
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
    M: ContextConstructor,
    A: Value,
{
    pub fn new(inner: WrappedFn<S, M::Type<(A, S)>>) -> Self {
        Self(inner)
    }

    pub fn run_tr(trans: impl Borrow<Self>, state: S) -> M::Type<(A, S)> {
        (trans.borrow().0)(state)
    }
}

impl<S, M, A> StateT<S, M, A>
where
    S: Value,
    M: Monad,
    A: Value,
{
    pub fn state<G>(run: G) -> Self
    where
        G: Into<WrappedFn<S, (A, S)>>,
    {
        StackedStateTInstance::state(run)
    }

    pub fn eval_tr(trans: impl Borrow<Self>, state: S) -> M::Type<A> {
        M::fmap(&(|(x, _)| x), Self::run_tr(trans, state))
    }

    pub fn exec_tr(trans: impl Borrow<Self>, state: S) -> M::Type<S> {
        M::fmap(&(|(_, s)| s), Self::run_tr(trans, state))
    }
}

impl<S, M, A, G> From<G> for StateT<S, M, A>
where
    S: Value,
    M: Monad,
    A: Value,
    G: Into<WrappedFn<S, (A, S)>>,
{
    fn from(func: G) -> Self {
        Self::state(func)
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
    fn lift<M, A>(mx: M::Type<A>) -> Self::Type<M, A>
    where
        M: Monad,
        A: Value,
        Self::Stacked<M>: Monad<Type<A> = Self::Type<M, A>>,
    {
        StateT(WrappedFn::from(move |s: S| {
            M::fmap(WrappedFn::from(move |x| (x, s.clone())), mx.clone())
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
