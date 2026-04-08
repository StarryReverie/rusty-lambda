use std::ops::ControlFlow;

use crate::base::function::ConcurrentFn;
use crate::base::value::{StaticConcurrent, Value};
use crate::control::structure::foldable::Foldable;
use crate::data::either::{Either, EitherInstance};

impl<E> Foldable for EitherInstance<E>
where
    E: StaticConcurrent,
{
    fn try_foldr<A, B, F, T>(accum: F, try_break: T, init: B, container: Self::Type<A>) -> B
    where
        A: Value,
        B: Value,
        F: for<'a> Value<View<'a>: ConcurrentFn<A, Output: ConcurrentFn<B, Output = B>>>,
        T: for<'a> ConcurrentFn<A, Output = ControlFlow<B, A>>,
    {
        match container {
            Either::Right(x) => match try_break.call(x) {
                ControlFlow::Continue(x) => accum.view().call(x).call(init),
                ControlFlow::Break(res) => res,
            },
            Either::Left(_) => init,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::{Curry, WrappedFn};
    use crate::control::structure::foldable::Foldable;
    use crate::control::structure::functor::fmap;
    use crate::data::list::List;

    use super::*;

    #[test]
    fn test_foldable_fold_fold_map_id_law() {
        let xs = Either::<List<i32>, List<i32>>::Right(List::from(vec![1, 2]));
        let lhs = EitherInstance::fold(xs.clone());
        let rhs = EitherInstance::fold_map(WrappedFn::from(|x| x), xs);
        assert_eq!(lhs, rhs);

        let xs = Either::<List<i32>, List<i32>>::Left(List::singleton(0));
        let lhs = EitherInstance::fold(xs.clone());
        let rhs = EitherInstance::fold_map(WrappedFn::from(|x| x), xs);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_foldable_fold_map_fmap_fusion_law() {
        let to_monoid = WrappedFn::from(|x| List::singleton(x));
        let elem_map = WrappedFn::from(|x| x + 1);
        let composed = to_monoid.clone().compose(elem_map.clone());

        let xs = Either::<&str, i32>::Right(2);
        let lhs = EitherInstance::fold_map(composed, xs.clone());
        let rhs = EitherInstance::fold_map(to_monoid, fmap(elem_map, xs));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_foldable_foldr_to_list_law() {
        let xs = Either::<&str, i32>::Right(1);
        let via_foldr =
            EitherInstance::foldr(WrappedFn::curry(List::cons), List::empty(), xs.clone());
        assert_eq!(via_foldr, List::from(vec![1]));

        let xs = Either::<&str, i32>::Left("err");
        let via_foldr = EitherInstance::foldr(WrappedFn::curry(List::cons), List::empty(), xs);
        assert_eq!(via_foldr, List::empty());
    }
}
