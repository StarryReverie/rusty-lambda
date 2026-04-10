use std::borrow::Borrow;
use std::marker::PhantomData;

use crate::base::function::{ConcurrentFn, WrappedFn, WrappedFnInstance};
use crate::base::value::{SimpleValue, Value};
use crate::control::context::ContextConstructor;
use crate::control::context::applicative::Applicative;
use crate::control::context::monad::Monad;
use crate::control::transformer::reader::MonadReader;
use crate::control::transformer::{MonadTrans, StackedMonadTrans, TransConstructor};

pub struct ReaderT<R, M, A>(pub(super) WrappedFn<R, M::Type<A>>)
where
    M: ContextConstructor,
    A: Value;

impl<R, M, A> ReaderT<R, M, A>
where
    M: ContextConstructor,
    A: Value,
{
    pub fn new(inner: WrappedFn<R, M::Type<A>>) -> Self {
        Self(inner)
    }

    pub fn run_tr(trans: impl Borrow<Self>, env: R) -> M::Type<A> {
        (trans.borrow().0)(env)
    }
}

impl<R, M, A> ReaderT<R, M, A>
where
    R: Value,
    M: Monad,
    A: Value,
{
    pub fn reader<G>(read: G) -> Self
    where
        G: Into<WrappedFn<R, A>>,
    {
        StackedReaderTInstance::reader(read.into())
    }
}

impl<R, M, A, G> From<G> for ReaderT<R, M, A>
where
    R: Value,
    M: Monad,
    A: Value,
    G: Into<WrappedFn<R, A>>,
{
    fn from(read: G) -> Self {
        StackedReaderTInstance::reader(read.into())
    }
}

impl<R, M, A> Clone for ReaderT<R, M, A>
where
    M: ContextConstructor,
    A: Value,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<R, M, A> SimpleValue for ReaderT<R, M, A>
where
    R: Value,
    M: ContextConstructor,
    A: Value,
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ReaderTInstance<R>(PhantomData<R>);

impl<R> TransConstructor for ReaderTInstance<R>
where
    R: Value,
{
    type Type<M, A>
        = ReaderT<R, M, A>
    where
        M: Monad,
        A: Value;

    type Stacked<M>
        = StackedReaderTInstance<R, M>
    where
        M: Monad;
}

impl<R> MonadTrans for ReaderTInstance<R>
where
    R: Value,
{
    fn lift<M, A>(mx: M::Type<A>) -> Self::Type<M, A>
    where
        M: Monad,
        A: Value,
        Self::Stacked<M>: Monad<Type<A> = Self::Type<M, A>>,
    {
        ReaderT(WrappedFnInstance::pure(mx))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StackedReaderTInstance<R, M>(PhantomData<(R, M)>);

impl<R, M> ContextConstructor for StackedReaderTInstance<R, M>
where
    R: Value,
    M: ContextConstructor,
{
    type Type<A>
        = ReaderT<R, M, A>
    where
        A: Value;
}

impl<R, M> StackedMonadTrans for StackedReaderTInstance<R, M>
where
    R: Value,
    M: Monad,
{
    type Transformer = ReaderTInstance<R>;
}

impl<R, M> MonadReader for StackedReaderTInstance<R, M>
where
    R: Value,
    M: Monad,
{
    type Environment = R;

    fn ask() -> Self::Type<Self::Environment> {
        ReaderT(WrappedFn::from(|env| M::pure(env)))
    }

    fn local<A, G>(localize: G, context: Self::Type<A>) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Environment, Output = Self::Environment>>,
    {
        ReaderT(WrappedFn::from(move |env| {
            ReaderT::run_tr(&context, localize.view().call(env))
        }))
    }
}
