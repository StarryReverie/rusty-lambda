use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::context::ContextConstructor;
use crate::control::structure::functor::{Functor, FunctorExt};

pub fn fmap<FA, B, G>(g: G, x: FA) -> <FA::Instance as ContextConstructor>::Type<B>
where
    FA: FunctorExt<Wrapped: Value> + Value,
    B: Value,
    G: for<'a> Value<View<'a>: ConcurrentFn<FA::Wrapped, Output = B>>,
{
    FA::Instance::fmap(g, x)
}

pub fn void<FA>(x: FA) -> <FA::Instance as ContextConstructor>::Type<()>
where
    FA: FunctorExt<Wrapped: Value> + Value,
{
    x.void()
}
