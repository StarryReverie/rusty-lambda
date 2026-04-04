use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::context::applicative::Applicative;
use crate::control::context::monad::Monad;
use crate::control::structure::functor::Functor;
use crate::control::structure::monoid::first::{First, FirstInstance};
use crate::data::maybe::{Maybe, MaybeInstance};

impl Functor for FirstInstance {
    fn fmap<A, B, G>(g: G, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        First(MaybeInstance::fmap(g, x.0))
    }
}

impl Applicative for FirstInstance {
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        First(MaybeInstance::pure(x))
    }

    fn apply<A, B, G>(g: Self::Type<G>, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        First(MaybeInstance::apply(g.0, x.0))
    }
}

impl Monad for FirstInstance {
    fn bind<A, B, G>(x: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        match x.0 {
            Maybe::Just(a) => g.view().call(a),
            Maybe::Nothing => First(Maybe::Nothing),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::WrappedFn;

    use super::*;

    #[test]
    fn test_fmap() {
        assert_eq!(
            FirstInstance::fmap(WrappedFn::from(|x| x + 1), First(Maybe::Just(3))),
            First(Maybe::Just(4)),
        );
        assert_eq!(
            FirstInstance::fmap(WrappedFn::from(|x: i32| x + 1), First(Maybe::Nothing)),
            First(Maybe::Nothing),
        );
    }

    #[test]
    fn test_pure() {
        assert_eq!(FirstInstance::pure(42), First(Maybe::Just(42)));
    }

    #[test]
    fn test_apply() {
        assert_eq!(
            FirstInstance::apply(
                First(Maybe::Just(WrappedFn::from(|x| x + 1))),
                First(Maybe::Just(1))
            ),
            First(Maybe::Just(2)),
        );
        assert_eq!(
            FirstInstance::apply(
                First(Maybe::Nothing::<WrappedFn<i32, i32>>),
                First(Maybe::Just(1))
            ),
            First(Maybe::Nothing),
        );
        assert_eq!(
            FirstInstance::apply(
                First(Maybe::Just(WrappedFn::from(|x: i32| x + 1))),
                First(Maybe::Nothing)
            ),
            First(Maybe::Nothing),
        );
    }

    #[test]
    fn test_bind() {
        assert_eq!(
            FirstInstance::bind(
                First(Maybe::Just(1)),
                WrappedFn::from(|x: i32| First(Maybe::Just(x + 1)))
            ),
            First(Maybe::Just(2)),
        );
        assert_eq!(
            FirstInstance::bind(
                First(Maybe::Nothing),
                WrappedFn::from(|x: i32| First(Maybe::Just(x + 1)))
            ),
            First(Maybe::Nothing),
        );
    }
}
