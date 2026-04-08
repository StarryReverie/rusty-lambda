use crate::base::function::{ConcurrentFn, WrappedFn};
use crate::base::value::Value;
use crate::control::context::applicative::{Applicative, ApplicativeExt};
use crate::control::structure::traversable::Traversable;
use crate::data::either::{Either, EitherInstance};

impl<E> Traversable for EitherInstance<E>
where
    E: Value,
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
            Either::Right(x) => F::fmap(WrappedFn::from(Self::pure), map.view().call(x)),
            Either::Left(e) => F::pure(Either::Left(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::{Curry, WrappedFn};
    use crate::control::structure::foldable::Foldable;
    use crate::control::structure::functor::identity::{Identity, IdentityInstance};
    use crate::data::maybe::{Maybe, MaybeInstance};

    use super::*;

    #[test]
    fn test_traverse() {
        let res = EitherInstance::<&str>::context(MaybeInstance)
            .traverse(WrappedFn::from(|x| Maybe::Just(x * 2)))
            .over(Either::Right(3));
        assert_eq!(res, Maybe::Just(Either::Right(6)));

        let res: Maybe<Either<&str, i32>> = EitherInstance::context(MaybeInstance)
            .traverse(WrappedFn::from(|x: i32| Maybe::Just(x * 2)))
            .over(Either::Left("err"));
        assert_eq!(res, Maybe::Just(Either::Left("err")));
    }

    #[test]
    fn test_traverse_effect_fail() {
        let res: Maybe<Either<&str, i32>> = EitherInstance::context(MaybeInstance)
            .traverse(WrappedFn::from(|_| Maybe::<i32>::Nothing))
            .over(Either::<&str, i32>::Right(3));
        assert_eq!(res, Maybe::Nothing);
    }

    #[test]
    fn test_traverse_identity_law() {
        let res = EitherInstance::<&str>::context(IdentityInstance)
            .traverse(WrappedFn::from(Identity))
            .over(Either::Right(42));
        assert_eq!(res, Identity(Either::Right(42)));

        let res = EitherInstance::<&str>::context(IdentityInstance)
            .traverse(WrappedFn::from(|x: i32| Identity(x)))
            .over(Either::Left("err"));
        assert_eq!(res, Identity(Either::Left("err")));
    }

    #[test]
    fn test_traverse_composition_law() {
        let xs = Either::<&str, i32>::Right(3);
        let g = WrappedFn::from(|x| x * 2);
        let h = WrappedFn::from(|x| x + 1);
        let composed = h.clone().compose(g.clone());

        let lhs = EitherInstance::<&str>::context(MaybeInstance)
            .traverse(WrappedFn::from(move |x| Maybe::Just(composed(x))))
            .over(xs.clone());
        let rhs = EitherInstance::<&str>::context(MaybeInstance)
            .traverse(WrappedFn::from(move |x| Maybe::Just(h(g(x)))))
            .over(xs);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_traverse_pure_law() {
        let xs = Either::<&str, i32>::Right(42);
        let res = EitherInstance::<&str>::context(MaybeInstance)
            .traverse(WrappedFn::from(Maybe::Just))
            .over(xs.clone());
        assert_eq!(res, Maybe::Just(xs));

        let xs = Either::<&str, i32>::Left("err");
        let res: Maybe<Either<&str, i32>> = EitherInstance::context(MaybeInstance)
            .traverse(WrappedFn::from(Maybe::Just))
            .over(xs);
        assert_eq!(res, Maybe::Just(Either::Left("err")));
    }

    #[test]
    fn test_traverse_foldable_consistency() {
        let xs = Either::<&str, i32>::Right(5);
        let via_traverse = EitherInstance::<&str>::context(IdentityInstance)
            .traverse(WrappedFn::from(Identity))
            .over(xs.clone());
        let via_foldr = EitherInstance::foldr(WrappedFn::curry(|x, acc| x + acc), 0, xs);
        assert_eq!(Identity::run(via_traverse), Either::Right(via_foldr));
    }
}
