use std::ops::ControlFlow;

use crate::base::function::{ConcurrentFn, Curry, WrappedFn};
use crate::base::hkt::TypeConstructor1;
use crate::base::value::Value;
use crate::control::structure::monoid::Monoid;
use crate::control::structure::monoid::all::All;
use crate::control::structure::monoid::any::Any;
use crate::control::structure::semigroup::Semigroup;
use crate::data::list::List;
use crate::data::maybe::Maybe;

pub trait Foldable: TypeConstructor1 {
    fn try_foldr<A, B, F, T>(accum: F, try_break: T, init: B, container: Self::Type<A>) -> B
    where
        A: Value,
        B: Value,
        F: for<'a> Value<View<'a>: ConcurrentFn<A, Output: ConcurrentFn<B, Output = B>>>,
        T: for<'a> ConcurrentFn<A, Output = ControlFlow<B, A>>;

    fn fold<M>(container: Self::Type<M>) -> M
    where
        M: Value + Monoid,
    {
        Self::foldr(
            WrappedFn::curry(Semigroup::associate),
            Monoid::empty(),
            container,
        )
    }

    fn fold_map<A, M, F>(map: F, container: Self::Type<A>) -> M
    where
        A: Value,
        M: Value + Monoid,
        F: for<'a> Value<View<'a>: ConcurrentFn<A, Output = M>>,
    {
        Self::foldr(
            WrappedFn::curry(move |x, a| map.view().call(x).associate(a)),
            Monoid::empty(),
            container,
        )
    }

    fn foldr<A, B, F>(accum: F, init: B, container: Self::Type<A>) -> B
    where
        A: Value,
        B: Value,
        F: for<'a> Value<View<'a>: ConcurrentFn<A, Output: ConcurrentFn<B, Output = B>>>,
    {
        Self::try_foldr(accum, |x| ControlFlow::Continue(x), init, container)
    }

    fn foldr1<A, F>(accum: F, container: Self::Type<A>) -> Maybe<A>
    where
        A: Value,
        F: for<'a> Value<View<'a>: ConcurrentFn<A, Output: ConcurrentFn<A, Output = A>>>,
    {
        Self::foldr(
            WrappedFn::curry(move |x, a| match a {
                Maybe::Just(a) => Maybe::Just(accum.view().call(x).call(a)),
                Maybe::Nothing => Maybe::Just(x),
            }),
            Maybe::Nothing,
            container,
        )
    }

    fn to_list<A>(container: Self::Type<A>) -> List<A>
    where
        A: Value,
    {
        Self::foldr(WrappedFn::curry(List::cons), List::empty(), container)
    }

    fn null<A>(container: Self::Type<A>) -> bool
    where
        A: Value,
    {
        Self::try_foldr(
            WrappedFn::curry(|_, _| false),
            |_| ControlFlow::Break(false),
            true,
            container,
        )
    }

    fn length<A>(container: Self::Type<A>) -> usize
    where
        A: Value,
    {
        Self::foldr(WrappedFn::curry(|_, a| a + 1), 0, container)
    }

    fn any<A, P>(predicate: P, container: Self::Type<A>) -> bool
    where
        A: Value,
        P: for<'a> ConcurrentFn<&'a A, Output = bool>,
    {
        matches!(Self::find(predicate, container), Maybe::Just(_))
    }

    fn all<A, P>(predicate: P, container: Self::Type<A>) -> bool
    where
        A: Value,
        P: for<'a> ConcurrentFn<&'a A, Output = bool>,
    {
        !Self::any(move |x: &A| !predicate.call(x), container)
    }

    fn find<A, P>(predicate: P, container: Self::Type<A>) -> Maybe<A>
    where
        A: Value,
        P: for<'a> ConcurrentFn<&'a A, Output = bool>,
    {
        Self::try_foldr(
            WrappedFn::curry(|_, a| a),
            move |x| {
                if predicate.call(&x) {
                    ControlFlow::Break(Maybe::Just(x))
                } else {
                    ControlFlow::Continue(x)
                }
            },
            Maybe::Nothing,
            container,
        )
    }

    fn elem<A>(target: A, container: Self::Type<A>) -> bool
    where
        A: Value + PartialEq,
    {
        Self::find(move |x: &A| *x == target, container) != Maybe::Nothing
    }

    fn not_elem<A>(target: A, container: Self::Type<A>) -> bool
    where
        A: Value + PartialEq,
    {
        !Self::elem(target, container)
    }

    fn and(container: Self::Type<bool>) -> bool {
        Self::fold_map(WrappedFn::from(All), container).0
    }

    fn or(container: Self::Type<bool>) -> bool {
        Self::fold_map(WrappedFn::from(Any), container).0
    }

    fn concat<A>(container: Self::Type<List<A>>) -> List<A>
    where
        A: Value,
    {
        Self::fold(container)
    }

    fn concat_map<A, B, F>(map: F, container: Self::Type<A>) -> List<B>
    where
        A: Value,
        B: Value,
        F: for<'a> Value<View<'a>: ConcurrentFn<A, Output = List<B>>>,
    {
        Self::fold_map(map, container)
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::{Curry, WrappedFn};
    use crate::data::list::{List, ListInstance};

    use super::*;

    #[test]
    fn test_try_foldr_short_circuits() {
        let xs = List::from(vec![1, 2, 3, 4, 5]);
        let result = ListInstance::try_foldr(
            WrappedFn::curry(|x, a| x + a),
            |x| {
                if x == 3 {
                    ControlFlow::Break(x)
                } else {
                    ControlFlow::Continue(x)
                }
            },
            0,
            xs,
        );
        assert_eq!(result, 6);
    }

    #[test]
    fn test_foldr() {
        let sum = ListInstance::foldr(WrappedFn::curry(|x: i32, a: i32| x + a), 0, List::empty());
        assert_eq!(sum, 0);

        let xs = List::from(vec![1, 2, 3]);
        let sum = ListInstance::foldr(WrappedFn::curry(|x, a| x + a), 0, xs);
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_foldr_cons_order() {
        let xs = List::from(vec![1, 2, 3]);
        let result = ListInstance::foldr(WrappedFn::curry(|x, a| x + a * 10), 0, xs);
        assert_eq!(result, 321);
    }

    #[test]
    fn test_foldr1() {
        let result = ListInstance::foldr1(WrappedFn::curry(|x: i32, a: i32| x + a), List::empty());
        assert_eq!(result, Maybe::Nothing);

        let xs = List::from(vec![1, 2, 3]);
        let result = ListInstance::foldr1(WrappedFn::curry(|x, a| x + a), xs);
        assert_eq!(result, Maybe::Just(6));
    }

    #[test]
    fn test_foldr1_cons_order() {
        let xs = List::from(vec![1, 2, 3]);
        let result = ListInstance::foldr1(WrappedFn::curry(|x, a| x + a * 10), xs);
        assert_eq!(result, Maybe::Just(321));
    }

    #[test]
    fn test_to_list() {
        let xs = List::from(vec![1, 2, 3]);
        assert_eq!(ListInstance::to_list(xs), List::from(vec![1, 2, 3]));
        assert_eq!(ListInstance::to_list(List::<i32>::empty()), List::empty());
    }

    #[test]
    fn test_null() {
        assert!(ListInstance::null(List::<i32>::empty()));
        assert!(!ListInstance::null(List::singleton(1)));
        assert!(!ListInstance::null(List::from(vec![1, 2, 3])));
    }

    #[test]
    fn test_length() {
        assert_eq!(ListInstance::length(List::<i32>::empty()), 0);
        assert_eq!(ListInstance::length(List::singleton(1)), 1);
        assert_eq!(ListInstance::length(List::from(vec![1, 2, 3])), 3);
    }

    #[test]
    fn test_any_all() {
        let xs = List::from(vec![1, 2, 3, 4, 5]);
        assert!(ListInstance::any(|x: &i32| *x == 3, xs.clone()));
        assert!(!ListInstance::any(|x: &i32| *x > 10, xs.clone()));
        assert!(ListInstance::all(|x: &i32| *x > 0, xs.clone()));
        assert!(!ListInstance::all(|x: &i32| *x > 2, xs));
    }

    #[test]
    fn test_find() {
        let xs = List::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(
            ListInstance::find(|x: &i32| *x == 3, xs.clone()),
            Maybe::Just(3)
        );
        assert_eq!(ListInstance::find(|x: &i32| *x == 99, xs), Maybe::Nothing);
        assert_eq!(
            ListInstance::find(|x: &i32| *x == 42, List::empty()),
            Maybe::Nothing
        );
    }

    #[test]
    fn test_elem() {
        let xs = List::from(vec![1, 2, 3]);
        assert!(ListInstance::elem(2, xs.clone()));
        assert!(!ListInstance::elem(5, xs));
        assert!(!ListInstance::elem(1, List::empty()));
    }

    #[test]
    fn test_and_or() {
        assert!(ListInstance::and(List::from(vec![true, true, true])));
        assert!(!ListInstance::and(List::from(vec![true, false, true])));
        assert!(ListInstance::and(List::empty()));
        assert!(ListInstance::or(List::from(vec![false, true, false])));
        assert!(!ListInstance::or(List::from(vec![false, false, false])));
        assert!(!ListInstance::or(List::empty()));
    }

    #[test]
    fn test_concat() {
        let xs = List::from(vec![
            List::from(vec![1, 2]),
            List::empty(),
            List::from(vec![3, 4, 5]),
        ]);
        assert_eq!(ListInstance::concat(xs), List::from(vec![1, 2, 3, 4, 5]));
        assert_eq!(
            ListInstance::concat(List::<List<i32>>::empty()),
            List::empty()
        );
    }

    #[test]
    fn test_concat_map() {
        let xs = List::from(vec![1, 2, 3]);
        let result =
            ListInstance::concat_map(WrappedFn::from(|x: i32| List::from(vec![x, x * 10])), xs);
        assert_eq!(result, List::from(vec![1, 10, 2, 20, 3, 30]));
    }
}
