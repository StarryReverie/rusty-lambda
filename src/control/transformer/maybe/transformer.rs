use std::marker::PhantomData;

use crate::base::hkt::TypeConstructor1;
use crate::base::value::StaticConcurrent;
use crate::data::maybe::Maybe;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MaybeT<M, A>(pub M::Type<Maybe<A>>)
where
    M: TypeConstructor1,
    A: StaticConcurrent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MaybeTInstance<M>(PhantomData<M>);

impl<M> TypeConstructor1 for MaybeTInstance<M>
where
    M: TypeConstructor1 + 'static,
{
    type Type<A>
        = MaybeT<M, A>
    where
        A: StaticConcurrent;
}
