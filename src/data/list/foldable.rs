use std::ops::ControlFlow;

use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::context::ContextConstructor;
use crate::control::structure::foldable::Foldable;
use crate::data::list::ListInstance;
use crate::data::maybe::Maybe;

impl Foldable for ListInstance {
    fn try_foldr<A, B, F, T>(accum: F, try_break: T, init: B, container: Self::Type<A>) -> B
    where
        A: Value,
        B: Value,
        F: for<'a> Value<View<'a>: ConcurrentFn<A, Output: ConcurrentFn<B, Output = B>>>,
        T: ConcurrentFn<A, Output = ControlFlow<B, A>>,
    {
        fn foldr_impl<A: Value, B: Value>(
            accum: &impl for<'a> Value<View<'a>: ConcurrentFn<A, Output: ConcurrentFn<B, Output = B>>>,
            try_break: &impl ConcurrentFn<A, Output = ControlFlow<B, A>>,
            init: B,
            container: <ListInstance as ContextConstructor>::Type<A>,
        ) -> B {
            match container.decompose() {
                Maybe::Just((x, xs)) => match try_break.call(x) {
                    ControlFlow::Break(res) => res,
                    ControlFlow::Continue(x) => {
                        let res = foldr_impl(accum, try_break, init, xs);
                        accum.view().call(x).call(res)
                    }
                },
                Maybe::Nothing => init,
            }
        }
        foldr_impl(&accum, &try_break, init, container)
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::{ConcurrentFn, Curry, WrappedFn};
    use crate::control::structure::foldable::Foldable;
    use crate::control::structure::functor::fmap;
    use crate::data::list::List;
    use crate::data::list::ListInstance;

    #[test]
    fn test_foldable_fold_fold_map_id_law() {
        let xs = List::from(vec![
            List::from(vec![1]),
            List::from(vec![2, 3]),
            List::empty(),
            List::from(vec![4, 5]),
        ]);
        let lhs = ListInstance::fold(xs.clone());
        let rhs = ListInstance::fold_map(WrappedFn::from(|x| x), xs);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_foldable_fold_map_fmap_fusion_law() {
        let xs = List::from(vec![1, 2, 3]);
        let to_monoid = WrappedFn::from(|x| List::singleton(x));
        let elem_map = WrappedFn::from(|x| x + 1);
        let composed = to_monoid.clone().compose(elem_map.clone());
        let lhs = ListInstance::fold_map(composed, xs.clone());
        let rhs = ListInstance::fold_map(to_monoid, fmap(elem_map, xs));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_foldable_foldr_to_list_law() {
        let xs = List::from(vec![1, 2, 3]);
        let via_foldr =
            ListInstance::foldr(WrappedFn::curry(List::cons), List::empty(), xs.clone());
        assert_eq!(via_foldr, xs);
    }
}
