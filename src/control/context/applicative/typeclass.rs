use crate::base::function::{ConcurrentFn, Curry, Curryed2Fn, WrappedFn};
use crate::base::hkt::TypeConstructor1;
use crate::base::value::{StaticConcurrent, Value};
use crate::control::structure::functor::Functor;

pub trait Applicative: Functor {
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value;

    fn apply<A, B, G>(g: Self::Type<G>, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>;
}

pub trait ApplicativeExt {
    type Wrapped: StaticConcurrent;
    type Instance: Applicative<Type<Self::Wrapped> = Self>;

    fn pure(x: Self::Wrapped) -> Self
    where
        Self: Sized,
        Self::Wrapped: Value,
    {
        Self::Instance::pure(x)
    }

    fn apply<A, B>(
        self,
        x: <Self::Instance as TypeConstructor1>::Type<A>,
    ) -> <Self::Instance as TypeConstructor1>::Type<B>
    where
        A: Value,
        B: Value,
        Self: Sized,
        Self::Wrapped: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        Self::Instance::apply::<A, B, Self::Wrapped>(self, x)
    }

    fn then<B>(
        self,
        y: <Self::Instance as TypeConstructor1>::Type<B>,
    ) -> <Self::Instance as TypeConstructor1>::Type<B>
    where
        Self: Sized,
        Self::Wrapped: Value,
        B: Value,
        <Self::Instance as TypeConstructor1>::Type<B>: Value,
    {
        Self::Instance::apply::<_, _, WrappedFn<B, B>>(
            Self::Instance::fmap::<_, _, Curryed2Fn<Self::Wrapped, B, B>>(
                WrappedFn::curry(|_, y| y),
                self,
            ),
            y,
        )
    }

    fn before<B>(self, y: <Self::Instance as TypeConstructor1>::Type<B>) -> Self
    where
        Self: Sized,
        Self::Wrapped: Value,
        B: Value,
        <Self::Instance as TypeConstructor1>::Type<B>: Value,
    {
        Self::Instance::apply::<_, _, WrappedFn<B, Self::Wrapped>>(
            Self::Instance::fmap::<_, _, Curryed2Fn<Self::Wrapped, B, Self::Wrapped>>(
                WrappedFn::curry(|x, _| x),
                self,
            ),
            y,
        )
    }

    fn applied_by<B, G>(
        self,
        g: <Self::Instance as TypeConstructor1>::Type<G>,
    ) -> <Self::Instance as TypeConstructor1>::Type<B>
    where
        Self: Sized,
        Self::Wrapped: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Wrapped, Output = B>>,
        <Self::Instance as TypeConstructor1>::Type<B>: Value,
    {
        Self::Instance::apply::<_, _, WrappedFn<G, B>>(
            Self::Instance::fmap::<_, _, Curryed2Fn<Self::Wrapped, G, B>>(
                WrappedFn::curry(|a, f: G| f.view().call(a)),
                self,
            ),
            g,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::{Curry, WrappedFn};
    use crate::control::context::applicative::ApplicativeExt;
    use crate::control::structure::functor::LhsFunctorExt;
    use crate::data::maybe::Maybe;

    #[test]
    fn test_apply() {
        let f = WrappedFn::curry(|x, y| x + y)
            .fmap(Maybe::Just(1))
            .apply(Maybe::Just(2));
        assert_eq!(f, Maybe::Just(3));

        let f = WrappedFn::curry(|x, y: i32| x + y)
            .fmap(Maybe::Just(1))
            .apply(Maybe::Nothing);
        assert_eq!(f, Maybe::Nothing);
    }

    #[test]
    fn test_then() {
        assert_eq!(Maybe::Just(1).then(Maybe::Just(2)), Maybe::Just(2));
        let nothing: Maybe<i32> = Maybe::Nothing;
        assert_eq!(nothing.clone().then(Maybe::Just(2)), Maybe::Nothing);
        assert_eq!(Maybe::Just(1).then(nothing), Maybe::Nothing);
    }

    #[test]
    fn test_before() {
        assert_eq!(Maybe::Just(1).before(Maybe::Just(2)), Maybe::Just(1));
        let nothing: Maybe<i32> = Maybe::Nothing;
        assert_eq!(nothing.clone().before(Maybe::Just(2)), Maybe::Nothing);
        assert_eq!(Maybe::Just(1).before(nothing), Maybe::Nothing);
    }

    #[test]
    fn test_then_discards_left() {
        let res = WrappedFn::curry(|x, y: i32| x + y)
            .fmap(Maybe::Just(1))
            .then(Maybe::Just(99));
        assert_eq!(res, Maybe::Just(99));
    }

    #[test]
    fn test_before_discards_right() {
        let res = Maybe::Just(42).before(Maybe::Just(99));
        assert_eq!(res, Maybe::Just(42));

        let result = Maybe::<i32>::Nothing.before(Maybe::Just(99));
        assert_eq!(result, Maybe::Nothing);
    }

    #[test]
    fn test_applied_by() {
        let fa = Maybe::Just(1);
        let ff = Maybe::Just(WrappedFn::from(|x| x + 10));
        assert_eq!(fa.applied_by(ff), Maybe::Just(11));

        let nothing: Maybe<i32> = Maybe::Nothing;
        assert_eq!(
            nothing.applied_by(Maybe::Just(WrappedFn::from(|x| x + 10))),
            Maybe::Nothing
        );
        assert_eq!(
            Maybe::Just(1).applied_by(Maybe::<WrappedFn<i32, i32>>::Nothing),
            Maybe::Nothing
        );
    }
}
