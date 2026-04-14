use crate::base::function::{WrappedFn, constv};
use crate::base::value::Value;
use crate::control::context::alternative::{Alternative, AlternativeExt};
use crate::control::transformer::reader::{ReaderT, StackedReaderTInstance};

impl<R, M> Alternative for StackedReaderTInstance<R, M>
where
    R: Value,
    M: Alternative,
{
    fn fallback<A>() -> Self::Type<A>
    where
        A: Value,
    {
        ReaderT::new(constv(M::fallback()))
    }

    fn alt<A>(one: Self::Type<A>, another: Self::Type<A>) -> Self::Type<A>
    where
        A: Value,
    {
        ReaderT::new(WrappedFn::from(move |env: R| {
            M::alt(
                ReaderT::run_tr(&one, env.clone()),
                ReaderT::run_tr(&another, env),
            )
        }))
    }
}

impl<R, M, A> AlternativeExt for ReaderT<R, M, A>
where
    R: Value,
    M: Alternative,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedReaderTInstance<R, M>;
}
