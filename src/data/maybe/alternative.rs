use crate::base::value::{Concurrent, Value};
use crate::control::context::alternative::Alternative;
use crate::data::maybe::{Maybe, MaybeInstance};

impl Alternative for MaybeInstance {
    fn fallback<A>() -> Self::Type<A>
    where
        A: Concurrent,
    {
        Maybe::Nothing
    }

    fn alt<A>(one: Self::Type<A>, another: Self::Type<A>) -> Self::Type<A>
    where
        A: Value,
    {
        match (&one, &another) {
            (Maybe::Just(_), _) => one,
            _ => another,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::WrappedFn;
    use crate::control::context::applicative::Applicative;
    use crate::control::structure::functor::Functor;

    use super::*;

    #[test]
    fn test_alternative_left_identity_law() {
        let x = MaybeInstance::fchain(Maybe::Just(1))
            .alt(MaybeInstance::fallback())
            .eval();
        assert_eq!(x, Maybe::Just(1));

        let x: Maybe<i32> = MaybeInstance::ffallback()
            .alt(MaybeInstance::fallback())
            .eval();
        assert_eq!(x, Maybe::Nothing);
    }

    #[test]
    fn test_alternative_right_identity_law() {
        let x = MaybeInstance::ffallback::<i32>().alt(Maybe::Just(1)).eval();
        assert_eq!(x, Maybe::Just(1));

        let x: Maybe<i32> = MaybeInstance::ffallback()
            .alt(MaybeInstance::fallback())
            .eval();
        assert_eq!(x, Maybe::Nothing);
    }

    #[test]
    fn test_alternative_associativity_law() {
        let a = Maybe::Just(1);
        let b = Maybe::Just(2);
        let c = Maybe::Just(3);
        let lhs = MaybeInstance::alt(MaybeInstance::alt(a, b), c);
        let rhs = MaybeInstance::alt(a, MaybeInstance::alt(b, c));
        assert_eq!(lhs, rhs);

        let a = Maybe::Nothing;
        let b = Maybe::Just(2);
        let c = Maybe::Nothing;
        let lhs = MaybeInstance::alt(MaybeInstance::alt(a, b), c);
        let rhs = MaybeInstance::alt(a, MaybeInstance::alt(b, c));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_alternative_left_distributivity_law() {
        let f = WrappedFn::from(|x| x * 2);
        let a = Maybe::Just(1);
        let b = Maybe::Just(2);
        let lhs = MaybeInstance::fmap(f.clone(), MaybeInstance::alt(a.clone(), b.clone()));
        let rhs = MaybeInstance::alt(MaybeInstance::fmap(f.clone(), a), MaybeInstance::fmap(f, b));
        assert_eq!(lhs, rhs);

        let f = WrappedFn::from(|x| x * 2);
        let a = Maybe::Nothing;
        let b = Maybe::Just(2);
        let lhs = MaybeInstance::fmap(f.clone(), MaybeInstance::alt(a.clone(), b.clone()));
        let rhs = MaybeInstance::alt(MaybeInstance::fmap(f.clone(), a), MaybeInstance::fmap(f, b));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_alternative_right_distributivity_law() {
        let f = WrappedFn::from(|x| x + 1);
        let g = WrappedFn::from(|x| x * 2);
        let x = Maybe::Just(3);

        let lhs = MaybeInstance::apply(
            MaybeInstance::alt(Maybe::Just(f.clone()), Maybe::Just(g.clone())),
            x,
        );
        let rhs = MaybeInstance::alt(
            MaybeInstance::apply(Maybe::Just(f), x),
            MaybeInstance::apply(Maybe::Just(g.clone()), x),
        );
        assert_eq!(lhs, rhs);

        let lhs = MaybeInstance::apply(
            MaybeInstance::alt(Maybe::Nothing, Maybe::Just(g.clone())),
            x,
        );
        let rhs = MaybeInstance::alt(
            MaybeInstance::apply(Maybe::<WrappedFn<i32, i32>>::Nothing, x),
            MaybeInstance::apply(Maybe::Just(g), x),
        );
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_alternative_annihilativity_law() {
        let res = MaybeInstance::apply(Maybe::<WrappedFn<i32, i32>>::Nothing, Maybe::Just(3));
        assert_eq!(res, Maybe::Nothing);

        let res = MaybeInstance::apply(
            Maybe::Just(WrappedFn::from(|x| x + 1)),
            Maybe::<i32>::Nothing,
        );
        assert_eq!(res, Maybe::Nothing);
    }

    #[test]
    fn test_chained_alt() {
        let x = MaybeInstance::ffallback::<i32>()
            .alt(Maybe::Just(1))
            .alt(Maybe::Just(2))
            .eval();
        assert_eq!(x, Maybe::Just(1));

        let x = MaybeInstance::fchain(Maybe::Just(10))
            .alt(Maybe::Just(20))
            .alt(MaybeInstance::fallback())
            .eval();
        assert_eq!(x, Maybe::Just(10));
    }
}
