use std::marker::PhantomData;

use crate::base::function::WrappedFn;
use crate::base::value::{SimpleValue, Value};
use crate::control::context::ContextConstructor;
use crate::control::context::monad::Monad;
use crate::control::transformer::{MonadTrans, StackedMonadTrans, TransConstructor};
use crate::data::maybe::Maybe;

#[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MaybeT<M, A>(pub(super) M::Type<Maybe<A>>)
where
    M: ContextConstructor,
    A: Value;

impl<M, A> MaybeT<M, A>
where
    M: ContextConstructor,
    A: Value,
{
    pub fn run_tr(trans: Self) -> M::Type<Maybe<A>> {
        trans.0
    }
}

impl<M, A> Clone for MaybeT<M, A>
where
    M: ContextConstructor,
    A: Value,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<M, A> MaybeT<M, A>
where
    M: ContextConstructor,
    A: Value,
{
    pub fn run(trans: Self) -> M::Type<Maybe<A>> {
        trans.0
    }
}

impl<M, A> SimpleValue for MaybeT<M, A>
where
    M: ContextConstructor,
    A: Value,
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MaybeTInstance;

impl TransConstructor for MaybeTInstance {
    type Type<M, A>
        = MaybeT<M, A>
    where
        M: Monad,
        A: Value;

    type Stacked<M>
        = StackedMaybeTInstance<M>
    where
        M: Monad;
}

impl MonadTrans for MaybeTInstance {
    fn lift<M, A>(mx: M::Type<A>) -> Self::Type<M, A>
    where
        M: Monad,
        A: Value,
        Self::Stacked<M>: Monad<Type<A> = Self::Type<M, A>>,
    {
        MaybeT(M::fmap(WrappedFn::from(Maybe::Just), mx))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StackedMaybeTInstance<M>(PhantomData<M>);

impl<M> ContextConstructor for StackedMaybeTInstance<M>
where
    M: ContextConstructor,
{
    type Type<A>
        = MaybeT<M, A>
    where
        A: Value;
}

impl<M> StackedMonadTrans for StackedMaybeTInstance<M>
where
    M: Monad,
{
    type Transformer = MaybeTInstance;
}
