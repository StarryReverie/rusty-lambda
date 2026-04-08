use std::borrow::Borrow;

use crate::base::function::{ConcurrentFn, WrappedFn};
use crate::base::value::Value;
use crate::control::context::monad::MonadExt;
use crate::control::structure::functor::Functor;
use crate::data::list::List;
use crate::data::maybe::{Maybe, MaybeInstance};

pub fn maybe<A, B, G>(default: B, g: G, x: Maybe<A>) -> B
where
    A: Value,
    B: Value,
    G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
{
    let y = MaybeInstance::fmap(g, x);
    from_maybe(default, y)
}

pub fn is_just<A>(x: impl Borrow<Maybe<A>>) -> bool {
    matches!(x.borrow(), Maybe::Just(_))
}

pub fn is_nothing<A>(x: impl Borrow<Maybe<A>>) -> bool {
    matches!(x.borrow(), Maybe::Nothing)
}

pub fn from_just<A>(x: Maybe<A>) -> A {
    match x {
        Maybe::Just(x) => x,
        Maybe::Nothing => panic!("expected a `Maybe::Just(x)`, got `Maybe::Nothing`"),
    }
}

pub fn from_maybe<A>(default: A, x: Maybe<A>) -> A {
    match x {
        Maybe::Just(x) => x,
        Maybe::Nothing => default,
    }
}

pub fn list_to_maybe<A>(xs: List<A>) -> Maybe<A>
where
    A: Clone,
{
    match xs.decompose() {
        Maybe::Just((x, _)) => Maybe::Just(x),
        Maybe::Nothing => Maybe::Nothing,
    }
}

pub fn maybe_to_list<A>(x: Maybe<A>) -> List<A> {
    match x {
        Maybe::Just(x) => List::singleton(x),
        Maybe::Nothing => List::empty(),
    }
}

pub fn cat_maybes<A>(xs: List<Maybe<A>>) -> List<A>
where
    A: Value,
{
    xs.bind(WrappedFn::from(maybe_to_list))
}

pub fn map_maybes<A, B, G>(map: G, xs: List<A>) -> List<B>
where
    A: Value,
    B: Value,
    G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Maybe<B>>>,
{
    xs.bind(WrappedFn::from(move |x| maybe_to_list(map.view().call(x))))
}

#[cfg(test)]
mod tests {
    use crate::base::function::WrappedFn;
    use crate::data::list::List;

    use super::*;

    #[test]
    fn test_maybe() {
        assert_eq!(maybe(0, WrappedFn::from(|x| x * 2), Maybe::Just(3)), 6);
        assert_eq!(maybe(0, WrappedFn::from(|x: i32| x * 2), Maybe::Nothing), 0);
    }

    #[test]
    fn test_is_just_is_nothing() {
        let x = Maybe::Just(42);
        assert!(is_just(x));
        assert!(!is_nothing(x));

        let x = Maybe::<i32>::Nothing;
        assert!(!is_just(x));
        assert!(is_nothing(x));
    }

    #[test]
    fn test_from_just() {
        assert_eq!(from_just(Maybe::Just(42)), 42);
    }

    #[test]
    fn test_from_maybe() {
        assert_eq!(from_maybe("default", Maybe::Just("a")), "a");
        assert_eq!(from_maybe("default", Maybe::Nothing), "default");
    }

    #[test]
    fn test_list_to_maybe() {
        assert_eq!(list_to_maybe(List::from(vec![1, 2, 3])), Maybe::Just(1));
        assert_eq!(list_to_maybe(List::<i32>::empty()), Maybe::Nothing);
    }

    #[test]
    fn test_maybe_to_list() {
        assert_eq!(maybe_to_list(Maybe::Just(1)), List::from(vec![1]));
        assert_eq!(maybe_to_list(Maybe::<i32>::Nothing), List::empty());
    }

    #[test]
    fn test_cat_maybes() {
        let xs = List::from(vec![Maybe::Just(1), Maybe::Nothing, Maybe::Just(3)]);
        assert_eq!(cat_maybes(xs), List::from(vec![1, 3]));
    }

    #[test]
    fn test_map_maybes() {
        let xs = List::from(vec![1, 2, 3, 4]);
        let result = map_maybes(
            WrappedFn::from(|x| {
                if x % 2 == 0 {
                    Maybe::Just(x)
                } else {
                    Maybe::Nothing
                }
            }),
            xs,
        );
        assert_eq!(result, List::from(vec![2, 4]));
    }
}
