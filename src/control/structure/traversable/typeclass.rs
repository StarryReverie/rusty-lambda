use std::marker::PhantomData;

use crate::base::function::{ConcurrentFn, WrappedFn};
use crate::base::value::Value;
use crate::control::context::applicative::{Applicative, ApplicativeExt};
use crate::control::structure::foldable::Foldable;
use crate::control::structure::functor::Functor;

pub trait Traversable: Functor + Foldable {
    fn traverse<F, A, B, FB, G>(map: G, container: Self::Type<A>) -> F::Type<Self::Type<B>>
    where
        F: Applicative<Type<B> = FB>,
        A: Value,
        B: Value,
        FB: ApplicativeExt<Wrapped = B, Instance = F> + Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = FB>>;

    fn sequence<F, A, FA, FTA>(contexts: Self::Type<FA>) -> FTA
    where
        F: Applicative<Type<A> = FA> + Applicative<Type<Self::Type<A>> = FTA>,
        A: Value,
        FA: ApplicativeExt<Wrapped = A, Instance = F> + Value,
        FTA: ApplicativeExt<Wrapped = Self::Type<A>, Instance = F>,
    {
        Self::traverse(WrappedFn::from(|x| x), contexts)
    }

    fn context<F>(_tag: F) -> TraversableChain<F, Self>
    where
        F: Applicative,
        Self: Sized,
    {
        TraversableChain {
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TraversableChain<F, T> {
    _marker: PhantomData<(F, T)>,
}

impl<F, T> TraversableChain<F, T>
where
    F: Applicative,
    T: Traversable,
{
    pub fn traverse<A, B, FB, G>(self, map: G) -> MappedTraversableChain<F, T, A, B, FB, G>
    where
        A: Value,
        B: Value,
        FB: ApplicativeExt<Wrapped = B, Instance = F> + Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = FB>>,
    {
        MappedTraversableChain {
            map,
            _marker: PhantomData,
        }
    }

    pub fn sequence<A, FA>(self, contexts: T::Type<FA>) -> F::Type<T::Type<A>>
    where
        F: Applicative<Type<A> = FA>,
        A: Value,
        FA: ApplicativeExt<Wrapped = A, Instance = F> + Value,
        F::Type<T::Type<A>>: ApplicativeExt<Wrapped = T::Type<A>, Instance = F>,
    {
        T::sequence(contexts)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MappedTraversableChain<F, T, A, B, FB, G> {
    map: G,
    _marker: PhantomData<(F, T, A, B, FB)>,
}

impl<F, T, A, B, FB, G> MappedTraversableChain<F, T, A, B, FB, G>
where
    F: Applicative<Type<B> = FB>,
    T: Traversable,
    A: Value,
    B: Value,
    FB: ApplicativeExt<Wrapped = B, Instance = F> + Value,
    G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = FB>>,
{
    pub fn over(self, container: T::Type<A>) -> F::Type<T::Type<B>> {
        T::traverse(self.map, container)
    }
}

#[cfg(test)]
mod tests {
    use crate::data::list::{List, ListInstance};
    use crate::data::maybe::Maybe;

    use super::*;

    #[test]
    fn test_sequence() {
        let res = ListInstance::sequence(List::from(vec![
            Maybe::Just(1),
            Maybe::Just(2),
            Maybe::Just(3),
        ]));
        assert_eq!(res, Maybe::Just(List::from(vec![1, 2, 3])));

        let res = ListInstance::sequence(List::from(vec![
            Maybe::Just(1),
            Maybe::Nothing,
            Maybe::Just(3),
        ]));
        assert_eq!(res, Maybe::Nothing);

        let res: Maybe<List<i32>> = ListInstance::sequence(List::empty());
        assert_eq!(res, Maybe::Just(List::empty()));
    }
}
