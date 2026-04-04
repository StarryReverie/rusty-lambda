use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::functor::Functor;
use crate::data::list::{List, ListInstance};
use crate::data::maybe::Maybe;

impl Functor for ListInstance {
    fn fmap<A, B, G>(g: G, xs: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        match xs.decompose() {
            Maybe::Just((x, xs)) => {
                let y = g.view().call(x);
                List::cons(y, Self::fmap(g, xs))
            }
            Maybe::Nothing => List::empty(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::{ConcurrentFn, WrappedFn};
    use crate::base::value::arc;

    use super::*;

    #[test]
    fn test_fmap() {
        let xs: List<i32> = List::empty();
        let ys = ListInstance::fmap(&|x| x + 1, xs);
        assert_eq!(ys, List::empty());

        let xs = List::from(vec![1, 2, 3]);
        let ys = ListInstance::fmap(&|x| x + 1, xs);
        let expected = List::from(vec![2, 3, 4]);
        assert_eq!(ys, expected);
    }

    #[test]
    fn test_functor_identity_law() {
        let id = |x| x;

        let xs = List::from(vec![1, 2, 3]);
        assert_eq!(ListInstance::fmap(arc(id), xs.clone()), xs);

        let xs: List<i32> = List::empty();
        assert_eq!(ListInstance::fmap(arc(id), xs), List::empty());
    }

    #[test]
    fn test_functor_composition_law() {
        let h = WrappedFn::from(|x| (x as i64) * 2);
        let g = WrappedFn::from(|x| x + 3);
        let composed = (g.clone()).compose(h.clone());

        let xs = List::from(vec![1, 2]);
        let lhs = ListInstance::fmap(composed.clone(), xs.clone());
        let rhs = ListInstance::fmap(g.clone(), ListInstance::fmap(h.clone(), xs));
        assert_eq!(lhs, rhs);

        let xs: List<i32> = List::empty();
        let lhs = ListInstance::fmap(composed, xs.clone());
        let rhs = ListInstance::fmap(g, ListInstance::fmap(h, xs));
        assert_eq!(lhs, rhs);
    }
}
