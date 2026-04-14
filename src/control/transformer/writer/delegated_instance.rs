use crate::base::value::Value;
use crate::control::context::alternative::{Alternative, AlternativeExt};
use crate::control::structure::monoid::Monoid;
use crate::control::transformer::writer::{StackedWriterTInstance, WriterT};

impl<W, M> Alternative for StackedWriterTInstance<W, M>
where
    W: Monoid + Value,
    M: Alternative,
{
    fn fallback<A>() -> Self::Type<A>
    where
        A: Value,
    {
        WriterT::new(M::fallback())
    }

    fn alt<A>(one: Self::Type<A>, another: Self::Type<A>) -> Self::Type<A>
    where
        A: Value,
    {
        WriterT::new(M::alt(WriterT::run_tr(one), WriterT::run_tr(another)))
    }
}

impl<W, M, A> AlternativeExt for WriterT<W, M, A>
where
    W: Monoid + Value,
    M: Alternative,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedWriterTInstance<W, M>;
}
