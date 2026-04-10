use std::borrow::Borrow;

use crate::base::function::{ConcurrentFn, Curry, WrappedFn};
use crate::base::value::Value;
use crate::control::context::monad::MonadExt;
use crate::control::structure::foldable::Foldable;
use crate::data::either::Either;
use crate::data::list::{List, ListInstance};

pub fn either<A, B, C, GL, GR>(left: GL, right: GR, x: Either<A, B>) -> C
where
    GL: for<'a> Value<View<'a>: ConcurrentFn<A, Output = C>>,
    GR: for<'a> Value<View<'a>: ConcurrentFn<B, Output = C>>,
{
    match x {
        Either::Left(x) => left.view().call(x),
        Either::Right(x) => right.view().call(x),
    }
}

pub fn lefts<A, B>(xs: List<Either<A, B>>) -> List<A>
where
    A: Value,
    B: Value,
{
    xs.bind(&|x| match x {
        Either::Left(x) => List::singleton(x),
        Either::Right(_) => List::empty(),
    })
}

pub fn rights<A, B>(xs: List<Either<A, B>>) -> List<B>
where
    A: Value,
    B: Value,
{
    xs.bind(&|x| match x {
        Either::Left(_) => List::empty(),
        Either::Right(x) => List::singleton(x),
    })
}

pub fn is_left<A, B>(x: impl Borrow<Either<A, B>>) -> bool {
    matches!(x.borrow(), Either::Left(_))
}

pub fn is_right<A, B>(x: impl Borrow<Either<A, B>>) -> bool {
    matches!(x.borrow(), Either::Right(_))
}

pub fn from_left<A, B>(default: A, x: Either<A, B>) -> A {
    match x {
        Either::Left(x) => x,
        Either::Right(_) => default,
    }
}

pub fn from_right<A, B>(default: B, x: Either<A, B>) -> B {
    match x {
        Either::Left(_) => default,
        Either::Right(x) => x,
    }
}

pub fn partition_eithers<A, B>(xs: List<Either<A, B>>) -> (List<A>, List<B>)
where
    A: Value,
    B: Value,
{
    ListInstance::foldr(
        WrappedFn::curry(|x, (ls, rs)| match x {
            Either::Left(x) => (List::cons(x, ls), rs),
            Either::Right(x) => (ls, List::cons(x, rs)),
        }),
        (List::empty(), List::empty()),
        xs,
    )
}

#[cfg(test)]
mod tests {
    use crate::base::function::WrappedFn;

    use super::*;

    #[test]
    fn test_either() {
        let f = WrappedFn::from(|x: i32| x.to_string());
        let g = WrappedFn::from(|x: i32| x.to_string());
        assert_eq!(either(f.clone(), g.clone(), Either::Left(10)), "10");
        assert_eq!(either(f, g, Either::Right(3)), "3");
    }

    #[test]
    fn test_lefts() {
        let xs = List::from(vec![Either::Left(1), Either::Right("a"), Either::Left(2)]);
        assert_eq!(lefts(xs), List::from(vec![1, 2]));
    }

    #[test]
    fn test_rights() {
        let xs = List::from(vec![
            Either::Left(1),
            Either::Right("a"),
            Either::Right("b"),
        ]);
        assert_eq!(rights(xs), List::from(vec!["a", "b"]));
    }

    #[test]
    fn test_is_left_is_right() {
        let x = Either::<&str, i32>::Left("err");
        assert!(is_left(x) && !is_right(x));

        let x = Either::<&str, i32>::Right(42);
        assert!(!is_left(x) && is_right(x));
    }

    #[test]
    fn test_from_left_from_right() {
        let x = Either::Left("a");
        let y = Either::Right(42);
        assert_eq!(from_left("default", x), "a");
        assert_eq!(from_left("default", y), "default");
        assert_eq!(from_right(0, x), 0);
        assert_eq!(from_right(0, y), 42);
    }

    #[test]
    fn test_partition_eithers() {
        let xs = List::from(vec![
            Either::Left(1),
            Either::Right("a"),
            Either::Left(2),
            Either::Right("b"),
        ]);
        let (ls, rs) = partition_eithers(xs);
        assert_eq!(ls, List::from(vec![1, 2]));
        assert_eq!(rs, List::from(vec!["a", "b"]));
    }
}
