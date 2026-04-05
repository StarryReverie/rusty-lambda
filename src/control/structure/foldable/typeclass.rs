use std::ops::ControlFlow;

use crate::base::function::{ConcurrentFn, Curry, WrappedFn};
use crate::base::hkt::TypeConstructor1;
use crate::base::value::Value;
use crate::control::structure::monoid::all::All;
use crate::control::structure::monoid::any::Any;
use crate::control::structure::monoid::Monoid;
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
