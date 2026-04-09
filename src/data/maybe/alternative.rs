use crate::base::value::Value;
use crate::control::context::alternative::{Alternative, AlternativeExt};
use crate::data::maybe::{Maybe, MaybeInstance};

impl Alternative for MaybeInstance {
    fn fallback<A>() -> Self::Type<A>
    where
        A: Value,
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

impl<T> AlternativeExt for Maybe<T>
where
    T: Value,
{
    type Wrapped = T;
    type Instance = MaybeInstance;
}

#[cfg(test)]
mod tests {
    use crate::base::function::WrappedFn;
    use crate::control::context::applicative::ApplicativeExt;
    use crate::control::structure::functor::fmap;

    use super::*;

    #[test]
    fn test_alternative_left_identity_law() {
        let x = Maybe::Just(1).alt(MaybeInstance::fallback());
        assert_eq!(x, Maybe::Just(1));

        let x: Maybe<i32> = MaybeInstance::fallback().alt(MaybeInstance::fallback());
        assert_eq!(x, Maybe::Nothing);
    }

    #[test]
    fn test_alternative_right_identity_law() {
        let x = MaybeInstance::fallback::<i32>().alt(Maybe::Just(1));
        assert_eq!(x, Maybe::Just(1));

        let x: Maybe<i32> = MaybeInstance::fallback().alt(MaybeInstance::fallback());
        assert_eq!(x, Maybe::Nothing);
    }

    #[test]
    fn test_alternative_associativity_law() {
        let a = Maybe::Just(1);
        let b = Maybe::Just(2);
        let c = Maybe::Just(3);
        let lhs = a.alt(b).alt(c);
        let rhs = a.alt(b.alt(c));
        assert_eq!(lhs, rhs);

        let a = Maybe::Nothing;
        let b = Maybe::Just(2);
        let c = Maybe::Nothing;
        let lhs = a.alt(b).alt(c);
        let rhs = a.alt(b.alt(c));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_alternative_left_distributivity_law() {
        let f = WrappedFn::from(|x| x * 2);
        let a = Maybe::Just(1);
        let b = Maybe::Just(2);
        let lhs = fmap(f.clone(), a.clone().alt(b.clone()));
        let rhs = fmap(f.clone(), a).alt(fmap(f, b));
        assert_eq!(lhs, rhs);

        let f = WrappedFn::from(|x| x * 2);
        let a = Maybe::Nothing;
        let b = Maybe::Just(2);
        let lhs = fmap(f.clone(), a.clone().alt(b.clone()));
        let rhs = fmap(f.clone(), a).alt(fmap(f, b));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_alternative_right_distributivity_law() {
        let f = WrappedFn::from(|x| x + 1);
        let g = WrappedFn::from(|x| x * 2);
        let x = Maybe::Just(3);

        let lhs = Maybe::Just(f.clone()).alt(Maybe::Just(g.clone())).apply(x);
        let rhs = Maybe::Just(f).apply(x).alt(Maybe::Just(g.clone()).apply(x));
        assert_eq!(lhs, rhs);

        let lhs = Maybe::Nothing.alt(Maybe::Just(g.clone())).apply(x);
        let rhs = Maybe::<WrappedFn<i32, i32>>::Nothing
            .apply(x)
            .alt(Maybe::Just(g).apply(x));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_alternative_annihilativity_law() {
        let res = Maybe::<WrappedFn<i32, i32>>::Nothing.apply(Maybe::Just(3));
        assert_eq!(res, Maybe::Nothing);

        let res = Maybe::Just(WrappedFn::from(|x| x + 1)).apply(Maybe::<i32>::Nothing);
        assert_eq!(res, Maybe::Nothing);
    }

    #[test]
    fn test_chained_alt() {
        let x = MaybeInstance::fallback::<i32>()
            .alt(Maybe::Just(1))
            .alt(Maybe::Just(2));
        assert_eq!(x, Maybe::Just(1));

        let x = Maybe::Just(10)
            .alt(Maybe::Just(20))
            .alt(MaybeInstance::fallback());
        assert_eq!(x, Maybe::Just(10));
    }
}
