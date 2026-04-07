use crate::base::function::{ConcurrentFn, WrappedFn};
use crate::base::value::Value;
use crate::control::context::applicative::{Applicative, ApplicativeExt};
use crate::control::structure::traversable::Traversable;
use crate::data::maybe::{Maybe, MaybeInstance};

impl Traversable for MaybeInstance {
    fn traverse<F, A, B, FB, G>(map: G, container: Self::Type<A>) -> F::Type<Self::Type<B>>
    where
        F: Applicative<Type<B> = FB>,
        A: Value,
        B: Value,
        FB: ApplicativeExt<Wrapped = B, Instance = F> + Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = FB>>,
    {
        match container {
            Maybe::Just(x) => F::fmap(WrappedFn::from(Maybe::Just), map.view().call(x)),
            Maybe::Nothing => F::pure(Maybe::Nothing),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::{Curry, WrappedFn};
    use crate::control::structure::foldable::Foldable;
    use crate::control::structure::functor::fmap;
    use crate::control::structure::functor::identity::{Identity, IdentityInstance};

    use super::*;

    #[test]
    fn test_traverse() {
        let res = MaybeInstance::context(MaybeInstance)
            .traverse(WrappedFn::from(|x| Maybe::Just(x * 2)))
            .over(Maybe::Just(3));
        assert_eq!(res, Maybe::Just(Maybe::Just(6)));

        let res = MaybeInstance::context(MaybeInstance)
            .traverse(WrappedFn::from(|x: i32| Maybe::Just(x * 2)))
            .over(Maybe::Nothing);
        assert_eq!(res, Maybe::Just(Maybe::Nothing));
    }

    #[test]
    fn test_traverse_effect_fail() {
        let res: Maybe<Maybe<i32>> = MaybeInstance::context(MaybeInstance)
            .traverse(WrappedFn::from(|_| Maybe::Nothing))
            .over(Maybe::Just(3));
        assert_eq!(res, Maybe::Nothing);
    }

    #[test]
    fn test_traverse_identity_law() {
        let res = MaybeInstance::context(IdentityInstance)
            .traverse(WrappedFn::from(|x| Identity(x)))
            .over(Maybe::Just(42));
        assert_eq!(res, Identity(Maybe::Just(42)));

        let res = MaybeInstance::context(IdentityInstance)
            .traverse(WrappedFn::from(|x: i32| Identity(x)))
            .over(Maybe::Nothing);
        assert_eq!(res, Identity(Maybe::Nothing));
    }

    #[test]
    fn test_traverse_naturality_law() {
        let xs = Maybe::Just(3);

        let lhs = MaybeInstance::context(MaybeInstance)
            .traverse(WrappedFn::from(|x| Maybe::Just(x + 1)))
            .over(xs);
        let rhs = fmap(WrappedFn::from(|x| Maybe::Just(x + 1)), Maybe::Just(3));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_traverse_composition_law() {
        let xs = Maybe::Just(3);
        let f = WrappedFn::from(|x| Maybe::Just(x));
        let g = WrappedFn::from(|x| Maybe::Just(x * 2));

        let g2 = g.clone();
        let lhs = MaybeInstance::context(MaybeInstance)
            .traverse(WrappedFn::from(move |x| {
                MaybeInstance::context(MaybeInstance)
                    .traverse(g2.clone())
                    .over(f(x))
            }))
            .over(xs.clone());

        let rhs = MaybeInstance::context(MaybeInstance).traverse(g).over(xs);

        assert_eq!(lhs, Maybe::Just(rhs));
    }

    #[test]
    fn test_traverse_pure_law() {
        let xs = Maybe::Just(42);
        let res = MaybeInstance::context(MaybeInstance)
            .traverse(WrappedFn::from(Maybe::Just))
            .over(xs.clone());
        assert_eq!(res, Maybe::Just(xs));

        let res: Maybe<Maybe<i32>> = MaybeInstance::context(MaybeInstance)
            .traverse(WrappedFn::from(Maybe::Just))
            .over(Maybe::Nothing);
        assert_eq!(res, Maybe::Just(Maybe::Nothing));
    }

    #[test]
    fn test_traverse_foldable_consistency() {
        let xs = Maybe::Just(5);
        let via_traverse = MaybeInstance::context(IdentityInstance)
            .traverse(WrappedFn::from(|x| Identity(x)))
            .over(xs.clone());
        let via_foldr = MaybeInstance::foldr(WrappedFn::curry(|x, acc| x + acc), 0, xs);
        assert_eq!(Identity::run(via_traverse), Maybe::Just(via_foldr));
    }
}
