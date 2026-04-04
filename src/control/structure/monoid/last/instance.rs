use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::context::applicative::Applicative;
use crate::control::context::monad::Monad;
use crate::control::structure::functor::Functor;
use crate::control::structure::monoid::last::{Last, LastInstance};
use crate::data::maybe::{Maybe, MaybeInstance};

impl Functor for LastInstance {
    fn fmap<A, B, G>(g: G, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        Last(MaybeInstance::fmap(g, x.0))
    }
}

impl Applicative for LastInstance {
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        Last(MaybeInstance::pure(x))
    }

    fn apply<A, B, G>(g: Self::Type<G>, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        Last(MaybeInstance::apply(g.0, x.0))
    }
}

impl Monad for LastInstance {
    fn bind<A, B, G>(x: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        match x.0 {
            Maybe::Just(a) => g.view().call(a),
            Maybe::Nothing => Last(Maybe::Nothing),
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
            LastInstance::fmap(WrappedFn::from(|x| x + 1), Last(Maybe::Just(3))),
            Last(Maybe::Just(4)),
        );
        assert_eq!(
            LastInstance::fmap(WrappedFn::from(|x: i32| x + 1), Last(Maybe::Nothing)),
            Last(Maybe::Nothing),
        );
    }

    #[test]
    fn test_pure() {
        assert_eq!(LastInstance::pure(42), Last(Maybe::Just(42)));
    }

    #[test]
    fn test_apply() {
        assert_eq!(
            LastInstance::apply(
                Last(Maybe::Just(WrappedFn::from(|x| x + 1))),
                Last(Maybe::Just(1))
            ),
            Last(Maybe::Just(2)),
        );
        assert_eq!(
            LastInstance::apply(
                Last(Maybe::Nothing::<WrappedFn<i32, i32>>),
                Last(Maybe::Just(1))
            ),
            Last(Maybe::Nothing),
        );
        assert_eq!(
            LastInstance::apply(
                Last(Maybe::Just(WrappedFn::from(|x: i32| x + 1))),
                Last(Maybe::Nothing)
            ),
            Last(Maybe::Nothing),
        );
    }

    #[test]
    fn test_bind() {
        assert_eq!(
            LastInstance::bind(
                Last(Maybe::Just(1)),
                WrappedFn::from(|x: i32| Last(Maybe::Just(x + 1)))
            ),
            Last(Maybe::Just(2)),
        );
        assert_eq!(
            LastInstance::bind(
                Last(Maybe::Nothing),
                WrappedFn::from(|x: i32| Last(Maybe::Just(x + 1)))
            ),
            Last(Maybe::Nothing),
        );
    }
}
