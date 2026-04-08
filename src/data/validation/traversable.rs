use crate::base::function::{ConcurrentFn, WrappedFn};
use crate::base::value::Value;
use crate::control::context::applicative::{Applicative, ApplicativeExt};
use crate::control::structure::semigroup::Semigroup;
use crate::control::structure::traversable::Traversable;
use crate::data::validation::{Validation, ValidationInstance};

impl<E> Traversable for ValidationInstance<E>
where
    E: Semigroup + Value,
{
    fn traverse<F, A, B, FB, G>(map: G, container: Self::Type<A>) -> F::Type<Self::Type<B>>
    where
        F: Applicative<Type<B> = FB>,
        A: Value,
        B: Value,
        FB: ApplicativeExt<Wrapped = B, Instance = F> + Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = FB>>,
    {
        match container {
            Validation::Success(x) => F::fmap(WrappedFn::from(Self::pure), map.view().call(x)),
            Validation::Failure(e) => F::pure(Validation::Failure(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::{Curry, WrappedFn};
    use crate::control::structure::foldable::Foldable;
    use crate::control::structure::functor::identity::{Identity, IdentityInstance};
    use crate::data::list::List;
    use crate::data::maybe::{Maybe, MaybeInstance};

    use super::*;

    #[test]
    fn test_traverse() {
        let res = ValidationInstance::<List<i32>>::context(MaybeInstance)
            .traverse(WrappedFn::from(|x| Maybe::Just(x * 2)))
            .over(Validation::Success(3));
        assert_eq!(res, Maybe::Just(Validation::Success(6)));

        let res: Maybe<Validation<List<i32>, i32>> = ValidationInstance::context(MaybeInstance)
            .traverse(WrappedFn::from(|x: i32| Maybe::Just(x * 2)))
            .over(Validation::Failure(List::singleton(1)));
        assert_eq!(res, Maybe::Just(Validation::Failure(List::singleton(1))));
    }

    #[test]
    fn test_traverse_effect_fail() {
        let res: Maybe<Validation<List<i32>, i32>> = ValidationInstance::context(MaybeInstance)
            .traverse(WrappedFn::from(|_| Maybe::<i32>::Nothing))
            .over(Validation::<List<i32>, i32>::Success(3));
        assert_eq!(res, Maybe::Nothing);
    }

    #[test]
    fn test_traverse_identity_law() {
        let res = ValidationInstance::<List<i32>>::context(IdentityInstance)
            .traverse(WrappedFn::from(Identity))
            .over(Validation::Success(42));
        assert_eq!(res, Identity(Validation::Success(42)));

        let res = ValidationInstance::<List<i32>>::context(IdentityInstance)
            .traverse(WrappedFn::from(|x: i32| Identity(x)))
            .over(Validation::Failure(List::singleton(1)));
        assert_eq!(res, Identity(Validation::Failure(List::singleton(1))));
    }

    #[test]
    fn test_traverse_composition_law() {
        let xs = Validation::<List<i32>, i32>::Success(3);
        let g = WrappedFn::from(|x| x * 2);
        let h = WrappedFn::from(|x| x + 1);
        let composed = h.clone().compose(g.clone());

        let lhs = ValidationInstance::<List<i32>>::context(MaybeInstance)
            .traverse(WrappedFn::from(move |x| Maybe::Just(composed(x))))
            .over(xs.clone());
        let rhs = ValidationInstance::<List<i32>>::context(MaybeInstance)
            .traverse(WrappedFn::from(move |x| Maybe::Just(h(g(x)))))
            .over(xs);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_traverse_pure_law() {
        let xs = Validation::<List<i32>, i32>::Success(42);
        let res = ValidationInstance::<List<i32>>::context(MaybeInstance)
            .traverse(WrappedFn::from(Maybe::Just))
            .over(xs.clone());
        assert_eq!(res, Maybe::Just(xs));

        let xs = Validation::<List<i32>, i32>::Failure(List::singleton(1));
        let res = ValidationInstance::<List<i32>>::context(MaybeInstance)
            .traverse(WrappedFn::from(Maybe::Just))
            .over(xs);
        assert_eq!(res, Maybe::Just(Validation::Failure(List::singleton(1))));
    }

    #[test]
    fn test_traverse_foldable_consistency() {
        let xs = Validation::<List<i32>, i32>::Success(5);
        let via_traverse = ValidationInstance::<List<i32>>::context(IdentityInstance)
            .traverse(WrappedFn::from(Identity))
            .over(xs.clone());
        let via_foldr = ValidationInstance::foldr(WrappedFn::curry(|x, acc| x + acc), 0, xs);
        assert_eq!(Identity::run(via_traverse), Validation::Success(via_foldr));
    }
}
