use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::context::applicative::{Applicative, ApplicativeExt};
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

impl<T> ApplicativeExt for Maybe<T>
where
    T: Value,
{
    type Wrapped = T;
    type Instance = MaybeInstance;
}

#[cfg(test)]
mod tests {
    use crate::base::function::WrappedFn;
    use crate::base::value::arc;

    use super::*;

    #[test]
    fn test_pure() {
        assert_eq!(MaybeInstance::pure(42), Maybe::Just(42));
    }

    #[test]
    fn test_apply() {
        let f = Maybe::Just(&|x| x + 1).apply(Maybe::Just(1));
        assert_eq!(f, Maybe::Just(2));

        let f = Maybe::Nothing::<WrappedFn<i32, i32>>.apply(Maybe::Just(1));
        assert_eq!(f, Maybe::Nothing);

        let f = Maybe::Just(&|x: i32| x + 1).apply(Maybe::Nothing);
        assert_eq!(f, Maybe::Nothing);
    }

    #[test]
    fn test_applicative_identity_law() {
        let id = |x| x;

        let f = Maybe::Just(42);
        assert_eq!(Maybe::Just(arc(id)).apply(f), Maybe::Just(42),);

        let f: Maybe<i32> = Maybe::Nothing;
        assert_eq!(Maybe::Just(arc(id)).apply(f), Maybe::Nothing,);
    }

    #[test]
    fn test_applicative_homomorphism_law() {
        let h = |x| x * 2;
        assert_eq!(
            MaybeInstance::pure(arc(h)).apply(MaybeInstance::pure(3)),
            MaybeInstance::pure(h(3))
        );
    }

    #[test]
    fn test_applicative_interchange_law() {
        let h = |x| x + 10;
        let x = 5;

        let lhs = Maybe::Just(arc(h)).apply(MaybeInstance::pure(x));
        let rhs = MaybeInstance::pure(arc(move |g: fn(i32) -> i32| g(x))).apply(Maybe::Just(h));
        assert_eq!(lhs, rhs);

        let lhs = Maybe::<fn(i32) -> i32>::Nothing.apply(MaybeInstance::pure(x));
        let rhs = MaybeInstance::pure(arc(move |g: fn(i32) -> i32| g(x)))
            .apply(Maybe::<fn(i32) -> i32>::Nothing);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_applicative_composition_law() {
        let g = |x| x * 2;
        let h = |x| x + 3;
        let composed = g.compose(h);

        let lhs = Maybe::Just(composed.clone()).apply(Maybe::Just(4));
        let rhs = Maybe::Just(arc(g)).apply(Maybe::Just(arc(h)).apply(Maybe::Just(4)));
        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Maybe::Just(14));
    }
}
