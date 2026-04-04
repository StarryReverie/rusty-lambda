use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::applicative::Applicative;
use crate::data::maybe::{Maybe, MaybeInstance};

impl Applicative for MaybeInstance {
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        Maybe::Just(x)
    }

    fn apply<A, B, G>(g: Self::Type<G>, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        match (g, x) {
            (Maybe::Just(g), Maybe::Just(x)) => Maybe::Just(g.view().call(x)),
            _ => Maybe::Nothing,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::{Curry, WrappedFn};
    use crate::base::value::arc;

    use super::*;

    #[test]
    fn test_pure() {
        assert_eq!(MaybeInstance::pure(42), Maybe::Just(42));
    }

    #[test]
    fn test_apply() {
        let f = MaybeInstance::apply(Maybe::Just(&|x: i32| x + 1), Maybe::Just(1));
        assert_eq!(f, Maybe::Just(2));

        let f: Maybe<i32> = MaybeInstance::apply(Maybe::Nothing::<fn(i32) -> i32>, Maybe::Just(1));
        assert_eq!(f, Maybe::Nothing);

        let f: Maybe<i32> = MaybeInstance::apply(Maybe::Just(&|x: i32| x + 1), Maybe::Nothing);
        assert_eq!(f, Maybe::Nothing);
    }

    #[test]
    fn test_applicative_identity_law() {
        let id = |x| x;

        let f = Maybe::Just(42);
        assert_eq!(
            MaybeInstance::apply(Maybe::Just(arc(id)), f),
            Maybe::Just(42),
        );

        let f: Maybe<i32> = Maybe::Nothing;
        assert_eq!(
            MaybeInstance::apply(Maybe::Just(arc(id)), f),
            Maybe::Nothing,
        );
    }

    #[test]
    fn test_applicative_homomorphism_law() {
        let h = |x| x * 2;
        assert_eq!(
            MaybeInstance::apply(MaybeInstance::pure(arc(h)), MaybeInstance::pure(3)),
            MaybeInstance::pure(h(3))
        );
    }

    #[test]
    fn test_applicative_interchange_law() {
        let h = |x| x + 10;
        let x = 5;

        let lhs = MaybeInstance::apply(Maybe::Just(arc(h)), MaybeInstance::pure(x));
        let rhs = MaybeInstance::apply(
            MaybeInstance::pure(arc(move |g: fn(i32) -> i32| g(x))),
            Maybe::Just(h),
        );
        assert_eq!(lhs, rhs);

        let lhs = MaybeInstance::apply(Maybe::<fn(i32) -> i32>::Nothing, MaybeInstance::pure(x));
        let rhs = MaybeInstance::apply(
            MaybeInstance::pure(arc(move |g: fn(i32) -> i32| g(x))),
            Maybe::<fn(i32) -> i32>::Nothing,
        );
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_applicative_composition_law() {
        let g = |x| x * 2;
        let h = |x| x + 3;
        let composed = g.compose(h);

        let lhs = MaybeInstance::apply(Maybe::Just(composed.clone()), Maybe::Just(4));
        let rhs = MaybeInstance::apply(
            Maybe::Just(arc(g)),
            MaybeInstance::apply(Maybe::Just(arc(h)), Maybe::Just(4)),
        );
        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Maybe::Just(14));
    }

    #[test]
    fn test_chained_apply() {
        let add = WrappedFn::curry(|x, y| x + y);

        let f = MaybeInstance::apure(add.clone())
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
