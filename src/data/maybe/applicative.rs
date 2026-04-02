use std::borrow::Borrow;

use crate::base::value::{StaticConcurrent, Value};
use crate::control::applicative::Applicative;
use crate::data::maybe::{Maybe, MaybeInstance};

impl Applicative for MaybeInstance {
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        Maybe::Just(x)
    }

    fn apply<A, B, G, GI>(g: Self::Type<G>, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: Borrow<GI> + Value,
        GI: Fn(A) -> B + StaticConcurrent,
    {
        match (g, x) {
            (Maybe::Just(g), Maybe::Just(x)) => Maybe::Just((g.borrow())(x)),
            _ => Maybe::Nothing,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure() {
        assert_eq!(MaybeInstance::pure(42), Maybe::Just(42));
    }

    #[test]
    fn test_apply() {
        let f = MaybeInstance::apply(Maybe::Just(|x: i32| x + 1), Maybe::Just(1));
        assert_eq!(f, Maybe::Just(2));

        let f: Maybe<i32> = MaybeInstance::apply(Maybe::Nothing::<fn(i32) -> i32>, Maybe::Just(1));
        assert_eq!(f, Maybe::Nothing);

        let f: Maybe<i32> = MaybeInstance::apply(Maybe::Just(|x: i32| x + 1), Maybe::Nothing);
        assert_eq!(f, Maybe::Nothing);
    }

    #[test]
    fn test_applicative_identity_law() {
        let id = |x| x;

        let f = Maybe::Just(42);
        assert_eq!(MaybeInstance::apply(Maybe::Just(id), f), Maybe::Just(42));

        let f: Maybe<i32> = Maybe::Nothing;
        assert_eq!(MaybeInstance::apply(Maybe::Just(id), f), Maybe::Nothing);
    }

    #[test]
    fn test_applicative_homomorphism_law() {
        let h = |x| x * 2;
        assert_eq!(
            MaybeInstance::apply(MaybeInstance::pure(h), MaybeInstance::pure(3)),
            MaybeInstance::pure(h(3))
        );
    }

    #[test]
    fn test_applicative_interchange_law() {
        let h = |x| x + 10;
        let x = 5;

        let lhs = MaybeInstance::apply(Maybe::Just(h), MaybeInstance::pure(x));
        let rhs = MaybeInstance::apply(
            MaybeInstance::pure(move |g: fn(i32) -> i32| g(x)),
            Maybe::Just(h),
        );
        assert_eq!(lhs, rhs);

        let lhs = MaybeInstance::apply(Maybe::<fn(i32) -> i32>::Nothing, MaybeInstance::pure(x));
        let rhs = MaybeInstance::apply(
            MaybeInstance::pure(move |g: fn(i32) -> i32| g(x)),
            Maybe::<fn(i32) -> i32>::Nothing,
        );
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_applicative_composition_law() {
        let g = |x| x * 2;
        let h = |x| x + 3;
        fn compose<A, B, C>(
            g: impl Fn(B) -> C + Value,
            h: impl Fn(A) -> B + Value,
        ) -> impl Fn(A) -> C + Value {
            move |x| g(h(x))
        }

        let lhs = MaybeInstance::apply(Maybe::Just(compose(g, h)), Maybe::Just(4));
        let rhs = MaybeInstance::apply(
            Maybe::Just(g),
            MaybeInstance::apply(Maybe::Just(h), Maybe::Just(4)),
        );
        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Maybe::Just(14));
    }

    #[test]
    fn test_chained_apply() {
        let add = |x: i32| move |y: i32| x + y;

        let f = MaybeInstance::apure(add)
            .apply(Maybe::Just(1))
            .apply(Maybe::Just(2))
            .eval();
        assert_eq!(f, Maybe::Just(3));

        let f = MaybeInstance::apure(add)
            .apply(Maybe::Just(1))
            .apply(Maybe::Nothing)
            .eval();
        assert_eq!(f, Maybe::Nothing);
    }
}
