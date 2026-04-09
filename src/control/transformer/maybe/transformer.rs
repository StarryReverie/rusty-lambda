use std::marker::PhantomData;

use crate::base::value::StaticConcurrent;
use crate::control::context::ContextConstructor;
use crate::data::maybe::Maybe;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MaybeT<M, A>(pub M::Type<Maybe<A>>)
where
    M: ContextConstructor,
    A: StaticConcurrent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MaybeTInstance<M>(PhantomData<M>);

impl<M> ContextConstructor for MaybeTInstance<M>
where
    M: ContextConstructor + 'static,
{
    type Type<A>
        = MaybeT<M, A>
    where
        A: StaticConcurrent;
}
