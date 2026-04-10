use std::marker::PhantomData;

use crate::base::function::WrappedFn;
use crate::base::value::{SimpleValue, Value};
use crate::control::context::ContextConstructor;
use crate::control::context::applicative::Applicative;
use crate::control::structure::functor::Functor;

pub struct WriterT<W, M, A>(M::Type<(A, W)>)
where
    W: Value,
    M: ContextConstructor,
    A: Value;

impl<W, M, A> WriterT<W, M, A>
where
    W: Value,
    M: ContextConstructor,
    A: Value,
{
    pub fn new(inner: M::Type<(A, W)>) -> Self {
        Self(inner)
    }

    pub fn run_tr(trans: Self) -> M::Type<(A, W)> {
        trans.0
    }
}

impl<W, M, A> WriterT<W, M, A>
where
    W: Value,
    M: Functor,
    A: Value,
{
    pub fn eval_tr(trans: Self) -> M::Type<A> {
        M::fmap(WrappedFn::from(|(x, _)| x), Self::run_tr(trans))
    }

    pub fn exec_tr(trans: Self) -> M::Type<W> {
        M::fmap(WrappedFn::from(|(_, l)| l), Self::run_tr(trans))
    }
}

impl<W, M, A> WriterT<W, M, A>
where
    W: Value,
    M: Applicative,
    A: Value,
{
    pub fn writer(entries: (A, W)) -> Self {
        Self::new(M::pure(entries))
    }
}

impl<W, M, A> From<(A, W)> for WriterT<W, M, A>
where
    W: Value,
    M: Applicative,
    A: Value,
{
    fn from(entries: (A, W)) -> Self {
        Self::writer(entries)
    }
}

impl<W, M, A> Clone for WriterT<W, M, A>
where
    W: Value,
    M: ContextConstructor,
    A: Value,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<W, M, A> SimpleValue for WriterT<W, M, A>
where
    W: Value,
    M: ContextConstructor,
    A: Value,
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct WriterTInstance<W>(PhantomData<W>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StackedWriterTInstance<W, M>(PhantomData<(W, M)>);

impl<W, M> ContextConstructor for StackedWriterTInstance<W, M>
where
    W: Value,
    M: ContextConstructor,
{
    type Type<A>
        = WriterT<W, M, A>
    where
        A: Value;
}
