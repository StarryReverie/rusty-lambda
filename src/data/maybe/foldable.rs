use std::ops::ControlFlow;

use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::structure::foldable::Foldable;
use crate::data::maybe::{Maybe, MaybeInstance};

impl Foldable for MaybeInstance {
    fn try_foldr<A, B, F, T>(accum: F, try_break: T, init: B, container: Self::Type<A>) -> B
    where
        A: Value,
        B: Value,
        F: for<'a> Value<View<'a>: ConcurrentFn<A, Output: ConcurrentFn<B, Output = B>>>,
        T: ConcurrentFn<A, Output = ControlFlow<B, A>>,
    {
        match container {
            Maybe::Just(x) => match try_break.call(x) {
                ControlFlow::Break(res) => res,
                ControlFlow::Continue(x) => accum.view().call(x).call(init),
            },
            Maybe::Nothing => init,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::{ConcurrentFn, Curry, WrappedFn};
    use crate::control::structure::foldable::Foldable;
    use crate::control::structure::functor::fmap;
    use crate::data::list::List;
    use crate::data::maybe::{Maybe, MaybeInstance};

    #[test]
    fn test_foldable_fold_fold_map_id_law() {
        let lhs = MaybeInstance::fold(Maybe::Just(List::from(vec![1, 2])));
        let rhs =
            MaybeInstance::fold_map(WrappedFn::from(|x| x), Maybe::Just(List::from(vec![1, 2])));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_foldable_fold_map_fmap_fusion_law() {
        let to_monoid = WrappedFn::from(|x: i32| List::singleton(x));
        let elem_map = WrappedFn::from(|x: i32| x + 1);
        let composed = to_monoid.clone().compose(elem_map.clone());
        let lhs = MaybeInstance::fold_map(composed, Maybe::Just(2));
        let rhs = MaybeInstance::fold_map(to_monoid, fmap(elem_map, Maybe::Just(2)));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_foldable_foldr_to_list_law() {
        let xs = Maybe::Just(1);
        let via_foldr =
            MaybeInstance::foldr(WrappedFn::curry(List::cons), List::empty(), xs.clone());
        assert_eq!(via_foldr, List::from(vec![1]));
    }
}
