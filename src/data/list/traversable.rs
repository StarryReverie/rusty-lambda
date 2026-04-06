use crate::base::function::{ConcurrentFn, Curry, WrappedFn};
use crate::base::value::Value;
use crate::control::context::applicative::Applicative;
use crate::control::structure::traversable::Traversable;
use crate::data::list::{List, ListInstance};
use crate::data::maybe::Maybe;

impl Traversable for ListInstance {
    fn traverse<F, A, B, G>(tag: F, map: G, container: Self::Type<A>) -> F::Type<Self::Type<B>>
    where
        F: Applicative,
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = F::Type<B>>>,
    {
        match container.decompose() {
            Maybe::Just((x, xs)) => {
                let x = map.view().call(x);
                let xs = Self::traverse(tag, map, xs);
                F::apply(F::apply(F::pure(WrappedFn::curry(List::cons)), x), xs)
            }
            Maybe::Nothing => F::pure(List::empty()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::{ConcurrentFn, Curry, WrappedFn};
    use crate::control::structure::foldable::Foldable;
    use crate::control::structure::functor::Functor;
    use crate::control::structure::functor::identity::{Identity, IdentityInstance};
    use crate::data::maybe::MaybeInstance;

    use super::*;

    #[test]
    fn test_traverse() {
        let res = ListInstance::context(MaybeInstance)
            .traverse(WrappedFn::from(|x: i32| Maybe::Just(x + 1)))
            .over(List::empty());
        assert_eq!(res, Maybe::Just(List::empty()));

        let xs = List::from(vec![1, 2, 3]);
        let res = ListInstance::context(MaybeInstance)
            .traverse(WrappedFn::from(|x| Maybe::Just(x * 2)))
            .over(xs);
        assert_eq!(res, Maybe::Just(List::from(vec![2, 4, 6])));
    }

    #[test]
    fn test_traverse_effect_fail() {
        let xs = List::from(vec![1, 2, 3]);
        let res = ListInstance::context(MaybeInstance)
            .traverse(WrappedFn::from(|x| {
                if x == 2 {
                    Maybe::Nothing
                } else {
                    Maybe::Just(x)
                }
            }))
            .over(xs);
        assert_eq!(res, Maybe::Nothing);
    }

    #[test]
    fn test_traverse_identity_law() {
        let xs = List::from(vec![1, 2, 3]);

        let res = ListInstance::context(IdentityInstance)
            .traverse(WrappedFn::from(|x| Identity(x)))
            .over(xs.clone());
        assert_eq!(res, Identity(xs));

        let res = ListInstance::context(IdentityInstance)
            .traverse(WrappedFn::from(|x: i32| Identity(x)))
            .over(List::empty());
        assert_eq!(res, Identity(List::empty()));
    }

    #[test]
    fn test_traverse_naturality_law() {
        let xs = List::from(vec![1, 2, 3]);

        let via_traverse = ListInstance::context(MaybeInstance)
            .traverse(WrappedFn::from(|x| Maybe::Just(x + 1)))
            .over(xs.clone());
        let via_fmap_then_wrap = Maybe::Just(ListInstance::fmap(WrappedFn::from(|x| x + 1), xs));
        assert_eq!(via_traverse, via_fmap_then_wrap);
    }

    #[test]
    fn test_traverse_composition_law() {
        let xs = List::from(vec![1, 2]);
        let f = WrappedFn::from(|x| x * 2);
        let g = WrappedFn::from(|x| x + 1);

        let composed = f.clone().compose(g.clone());
        let via_composed = ListInstance::context(MaybeInstance)
            .traverse(WrappedFn::from(move |x| Maybe::Just(composed(x))))
            .over(xs.clone());

        let via_separate = ListInstance::context(MaybeInstance)
            .traverse(WrappedFn::from(move |x| Maybe::Just(f(g(x)))))
            .over(xs);

        assert_eq!(via_composed, via_separate);
    }

    #[test]
    fn test_traverse_foldable_consistency() {
        let xs = List::from(vec![1, 2, 3]);
        let via_traverse = ListInstance::context(IdentityInstance)
            .traverse(WrappedFn::from(|x: i32| Identity(x)))
            .over(xs.clone());
        assert_eq!(ListInstance::length(Identity::run(via_traverse)), 3);

        let sum = ListInstance::foldr(WrappedFn::curry(|x, a| x + a), 0, xs);
        assert_eq!(sum, 6);
    }
}
