use std::marker::PhantomData;

use crate::base::computation::Computation;
use crate::base::function::WrappedTcFn;
use crate::base::value::{SimpleValue, Value};
use crate::control::context::ContextConstructor;
use crate::control::context::applicative::Applicative;

pub struct ContT<R, M, A>(WrappedTcFn<WrappedTcFn<A, M::Type<R>>, M::Type<R>>)
where
    R: Value,
    M: ContextConstructor;

impl<R, M, A> ContT<R, M, A>
where
    R: Value,
    M: ContextConstructor,
{
    pub fn new(inner: WrappedTcFn<WrappedTcFn<A, M::Type<R>>, M::Type<R>>) -> Self {
        Self(inner)
    }
}

impl<R, M, A> ContT<R, M, A>
where
    R: Value,
    M: ContextConstructor,
    A: Value,
{
    pub fn run_tr(
        trans: Self,
        continuation: WrappedTcFn<A, M::Type<R>>,
    ) -> Computation<M::Type<R>> {
        Computation::monadic(move || (trans.0)(continuation))
    }
}

impl<R, M> ContT<R, M, R>
where
    R: Value,
    M: Applicative,
{
    pub fn eval_tr(trans: Self) -> Computation<M::Type<R>> {
        Self::run_tr(
            trans,
            WrappedTcFn::from(move |x| Computation::immediate(M::pure(x))),
        )
    }
}

impl<R, M, A> Clone for ContT<R, M, A>
where
    R: Value,
    M: ContextConstructor,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<R, M, A> SimpleValue for ContT<R, M, A>
where
    R: Value,
    M: ContextConstructor,
    A: Value,
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ContTInstance<R>(PhantomData<R>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StackedContTInstance<R, M>(PhantomData<(R, M)>);

impl<R, M> ContextConstructor for StackedContTInstance<R, M>
where
    R: Value,
    M: ContextConstructor,
{
    type Type<A>
        = ContT<R, M, A>
    where
        A: Value;
}
