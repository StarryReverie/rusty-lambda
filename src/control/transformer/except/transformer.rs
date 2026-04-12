use std::marker::PhantomData;

use crate::base::value::{SimpleValue, Value};
use crate::control::context::ContextConstructor;
use crate::control::context::applicative::Applicative;
use crate::control::context::monad::Monad;
use crate::control::transformer::{MonadTrans, StackedMonadTrans, TransConstructor};
use crate::data::either::Either;

#[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ExceptT<E, M, A>(M::Type<Either<E, A>>)
where
    E: Value,
    M: ContextConstructor,
    A: Value;

impl<E, M, A> ExceptT<E, M, A>
where
    E: Value,
    M: ContextConstructor,
    A: Value,
{
    pub fn new(inner: M::Type<Either<E, A>>) -> Self {
        Self(inner)
    }

    pub fn run_tr(trans: Self) -> M::Type<Either<E, A>> {
        trans.0
    }
}

impl<E, M, A> ExceptT<E, M, A>
where
    E: Value,
    M: Applicative,
    A: Value,
{
    pub fn except(either: Either<E, A>) -> Self {
        Self::new(M::pure(either))
    }
}

impl<E, M, A> SimpleValue for ExceptT<E, M, A>
where
    E: Value,
    M: ContextConstructor,
    A: Value,
{
}

impl<E, M, A> From<Either<E, A>> for ExceptT<E, M, A>
where
    E: Value,
    M: Applicative,
    A: Value,
{
    fn from(either: Either<E, A>) -> Self {
        Self::except(either)
    }
}

impl<E, M, A> Clone for ExceptT<E, M, A>
where
    E: Value,
    M: ContextConstructor,
    A: Value,
{
    fn clone(&self) -> Self {
        Self::new(self.0.clone())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ExceptTInstance<E>(PhantomData<E>);

impl<E> TransConstructor for ExceptTInstance<E>
where
    E: Value,
{
    type Type<M, A>
        = ExceptT<E, M, A>
    where
        M: Monad,
        A: Value;

    type Stacked<M>
        = StackedExceptTInstance<E, M>
    where
        M: Monad;
}

impl<E> MonadTrans for ExceptTInstance<E>
where
    E: Value,
{
    fn lift<M, A>(mx: M::Type<A>) -> Self::Type<M, A>
    where
        M: Monad,
        A: Value,
        Self::Stacked<M>: Monad<Type<A> = Self::Type<M, A>>,
    {
        ExceptT::new(M::fmap(&Either::Right, mx))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StackedExceptTInstance<E, M>(PhantomData<(E, M)>);

impl<E, M> ContextConstructor for StackedExceptTInstance<E, M>
where
    E: Value,
    M: ContextConstructor,
{
    type Type<A>
        = ExceptT<E, M, A>
    where
        A: Value;
}

impl<E, M> StackedMonadTrans for StackedExceptTInstance<E, M>
where
    E: Value,
    M: Monad,
{
    type Transformer = ExceptTInstance<E>;
}
