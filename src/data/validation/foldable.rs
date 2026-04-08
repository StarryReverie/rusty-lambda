use std::ops::ControlFlow;

use crate::base::function::ConcurrentFn;
use crate::base::value::{StaticConcurrent, Value};
use crate::control::structure::foldable::Foldable;
use crate::data::validation::{Validation, ValidationInstance};

impl<E> Foldable for ValidationInstance<E>
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
            Validation::Failure(_) => init,
            Validation::Success(x) => match try_break.call(x) {
                ControlFlow::Continue(x) => accum.view().call(x).call(init),
                ControlFlow::Break(res) => res,
            },
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
        let xs = Validation::<List<i32>, List<i32>>::Success(List::from(vec![1, 2]));
        let lhs = ValidationInstance::fold(xs.clone());
        let rhs = ValidationInstance::fold_map(WrappedFn::from(|x| x), xs);
        assert_eq!(lhs, rhs);

        let xs = Validation::<List<i32>, List<i32>>::Failure(List::singleton(0));
        let lhs = ValidationInstance::fold(xs.clone());
        let rhs = ValidationInstance::fold_map(WrappedFn::from(|x| x), xs);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_foldable_fold_map_fmap_fusion_law() {
        let to_monoid = WrappedFn::from(|x| List::singleton(x));
        let elem_map = WrappedFn::from(|x| x + 1);
        let composed = to_monoid.clone().compose(elem_map.clone());

        let xs = Validation::<List<i32>, i32>::Success(2);
        let lhs = ValidationInstance::fold_map(composed, xs.clone());
        let rhs = ValidationInstance::fold_map(to_monoid, fmap(elem_map, xs));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_foldable_foldr_to_list_law() {
        let xs = Validation::<List<i32>, i32>::Success(1);
        let via_foldr =
            ValidationInstance::foldr(WrappedFn::curry(List::cons), List::empty(), xs.clone());
        assert_eq!(via_foldr, List::from(vec![1]));

        let xs = Validation::<List<i32>, i32>::Failure(List::singleton(99));
        let via_foldr = ValidationInstance::foldr(WrappedFn::curry(List::cons), List::empty(), xs);
        assert_eq!(via_foldr, List::empty());
    }
}
