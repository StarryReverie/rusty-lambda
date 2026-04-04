use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::functor::Functor;
use crate::data::maybe::{Maybe, MaybeInstance};

pub fn maybe<A, B, G>(default: B, g: G, x: Maybe<A>) -> B
where
    A: Value,
    B: Value,
    G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
{
    let y = MaybeInstance::fmap(g, x);
    from_maybe(default, y)
}

pub fn from_maybe<A>(default: A, x: Maybe<A>) -> A {
    match x {
        Maybe::Just(x) => x,
        Maybe::Nothing => default,
    }
}

pub fn is_just<A>(x: Maybe<A>) -> bool {
    matches!(x, Maybe::Just(_))
}

pub fn is_nothing<A>(x: Maybe<A>) -> bool {
    !is_just(x)
}
