use crate::base::function::{ConcurrentFn, WrappedFn, WrappedFnInstance, id};
use crate::base::value::Value;
use crate::control::context::monad::Monad;

pub trait MonadReader: Monad {
    type Environment: Value;

    fn ask() -> Self::Type<Self::Environment>;

    fn local<A, G>(localize: G, context: Self::Type<A>) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Environment, Output = Self::Environment>>;

    fn reader<A, G>(map: G) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Environment, Output = A>>,
    {
        Self::fmap(map, Self::ask())
    }
}

impl<R> MonadReader for WrappedFnInstance<R>
where
    R: Value,
{
    type Environment = R;

    fn ask() -> Self::Type<Self::Environment> {
        id()
    }

    fn local<A, G>(localize: G, context: Self::Type<A>) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Environment, Output = Self::Environment>>,
    {
        WrappedFn::from(move |env| context(localize.view().call(env)))
    }
}
