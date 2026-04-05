use std::marker::PhantomData;

use crate::base::function::{ConcurrentFn, WrappedFn};
use crate::base::value::Value;
use crate::control::context::applicative::Applicative;
use crate::control::structure::foldable::Foldable;
use crate::control::structure::functor::Functor;

pub trait Traversable: Functor + Foldable {
    fn traverse<F, A, B, G>(tag: F, map: G, container: Self::Type<A>) -> F::Type<Self::Type<B>>
    where
        F: Applicative,
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = F::Type<B>>>;

    fn sequence<F, A>(tag: F, contexts: Self::Type<F::Type<A>>) -> F::Type<Self::Type<A>>
    where
        F: Applicative,
        A: Value,
        F::Type<A>: Value,
    {
        Self::traverse(tag, WrappedFn::from(|x| x), contexts)
    }

    fn context<F>(tag: F) -> TraversableChain<F, Self>
    where
        F: Applicative,
        Self: Sized,
    {
        TraversableChain {
            applicative_tag: tag,
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TraversableChain<F, T> {
    applicative_tag: F,
    _marker: PhantomData<T>,
}

impl<F, T> TraversableChain<F, T>
where
    F: Applicative,
    T: Traversable,
{
    pub fn traverse<A, B, G>(self, map: G) -> MappedTraversableChain<F, T, A, B, G>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = F::Type<B>>>,
    {
        MappedTraversableChain {
            applicative_tag: self.applicative_tag,
            map,
            _marker: PhantomData,
        }
    }

    pub fn sequence<A>(self, contexts: T::Type<F::Type<A>>) -> F::Type<T::Type<A>>
    where
        A: Value,
        F::Type<A>: Value,
    {
        T::sequence(self.applicative_tag, contexts)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MappedTraversableChain<F, T, A, B, G> {
    applicative_tag: F,
    map: G,
    _marker: PhantomData<(T, A, B)>,
}

impl<F, T, A, B, G> MappedTraversableChain<F, T, A, B, G>
where
    F: Applicative,
    T: Traversable,
    A: Value,
    B: Value,
    G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = F::Type<B>>>,
{
    pub fn over(self, container: T::Type<A>) -> F::Type<T::Type<B>> {
        T::traverse(self.applicative_tag, self.map, container)
    }
}
