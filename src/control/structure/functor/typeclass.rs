use crate::base::function::{ConcurrentFn, constv};
use crate::base::hkt::TypeConstructor1;
use crate::base::value::{StaticConcurrent, Value};

pub trait Functor: TypeConstructor1 {
    fn fmap<A, B, G>(g: G, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>;
}

pub trait FunctorExt {
    type Wrapped: StaticConcurrent;
    type Instance: Functor<Type<Self::Wrapped> = Self>;

    fn piped<B, G>(self, g: G) -> <Self::Instance as TypeConstructor1>::Type<B>
    where
        Self: Sized,
        Self::Wrapped: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Wrapped, Output = B>>,
    {
        Self::Instance::fmap::<Self::Wrapped, B, G>(g, self)
    }

    fn with<B>(self, y: B) -> <Self::Instance as TypeConstructor1>::Type<B>
    where
        Self: Sized,
        Self::Wrapped: Value,
        B: Value,
    {
        Self::Instance::fmap::<Self::Wrapped, B, _>(constv(y), self)
    }

    fn void(self) -> <Self::Instance as TypeConstructor1>::Type<()>
    where
        Self: Sized,
        Self::Wrapped: Value,
    {
        self.with(())
    }
}

pub trait LhsFunctorExt {
    fn to<FB>(self, y: FB) -> <FB::Instance as TypeConstructor1>::Type<Self>
    where
        Self: Value + Sized,
        FB: FunctorExt<Wrapped: Value> + Value,
    {
        y.with(self)
    }

    fn fmap<FA, B>(self, x: FA) -> <FA::Instance as TypeConstructor1>::Type<B>
    where
        Self: for<'a> Value<View<'a>: ConcurrentFn<FA::Wrapped, Output = B>> + Sized,
        FA: FunctorExt<Wrapped: Value> + Value,
        B: Value,
        <FA::Instance as TypeConstructor1>::Type<B>: Value,
    {
        FA::Instance::fmap(self, x)
    }
}

impl<T> LhsFunctorExt for T where T: Value {}

#[cfg(test)]
mod tests {
    use crate::base::value::arc;
    use crate::control::structure::functor::{FunctorExt, LhsFunctorExt, fmap};
    use crate::data::maybe::Maybe;

    #[test]
    fn test_piped() {
        assert_eq!(Maybe::Just(42).piped(arc(|x| x + 1)), Maybe::Just(43));
        let nothing: Maybe<i32> = Maybe::Nothing;
        assert_eq!(nothing.piped(arc(|x| x + 1)), Maybe::Nothing);
    }

    #[test]
    fn test_with() {
        assert_eq!(Maybe::Just(42).with("replaced"), Maybe::Just("replaced"));
        let nothing: Maybe<i32> = Maybe::Nothing;
        assert_eq!(nothing.with("replaced"), Maybe::Nothing);
    }

    #[test]
    fn test_void() {
        assert_eq!(Maybe::Just(42).void(), Maybe::Just(()));
        let nothing: Maybe<i32> = Maybe::Nothing;
        assert_eq!(nothing.void(), Maybe::Nothing);
    }

    #[test]
    fn test_to() {
        assert_eq!(42.to(Maybe::Just(0)), Maybe::Just(42));
        let nothing: Maybe<i32> = Maybe::Nothing;
        assert_eq!(42.to(nothing), Maybe::Nothing);
    }

    #[test]
    fn test_lhs_fmap() {
        assert_eq!(arc(|x| x * 2).fmap(Maybe::Just(21)), Maybe::Just(42));
        let nothing: Maybe<i32> = Maybe::Nothing;
        assert_eq!(arc(|x: i32| x * 2).fmap(nothing), Maybe::Nothing);
    }

    #[test]
    fn test_fmap_free_function() {
        assert_eq!(fmap(arc(|x| x + 5), Maybe::Just(10)), Maybe::Just(15));
        let nothing: Maybe<i32> = Maybe::Nothing;
        assert_eq!(fmap(arc(|x: i32| x + 5), nothing), Maybe::Nothing);
    }

    #[test]
    fn test_equivalence() {
        let g = arc(|x| x + 1);
        let fa = Maybe::Just(10);
        assert_eq!(fmap(g.clone(), fa.clone()), fa.piped(g.clone()));
        assert_eq!(fmap(g.clone(), fa), g.fmap(Maybe::Just(10)));
    }
}
