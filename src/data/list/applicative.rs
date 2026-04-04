use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::applicative::Applicative;
use crate::control::functor::Functor;
use crate::data::list::{List, ListInstance};
use crate::data::maybe::Maybe;

impl Applicative for ListInstance {
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        List::singleton(x)
    }

    fn apply<A, B, G>(gs: Self::Type<G>, xs: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        match gs.decompose() {
            Maybe::Just((g, gs)) => {
                let ys = ListInstance::fmap(g, xs.clone());
                ys.append(Self::apply(gs, xs))
            }
            Maybe::Nothing => List::empty(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::{Curry, WrappedFn, compose};
    use crate::base::value::arc;

    use super::*;

    #[test]
    fn test_pure() {
        assert_eq!(ListInstance::pure(42), List::singleton(42));
    }

    #[test]
    fn test_apply_nil_gs() {
        let gs: List<fn(i32) -> i32> = List::empty();
        let xs = List::from(vec![1, 2]);
        assert_eq!(ListInstance::apply(gs, xs), List::empty());
    }

    #[test]
    fn test_apply_nil_xs() {
        let gs = List::singleton(&|x: i32| x + 1);
        let xs: List<i32> = List::empty();
        assert_eq!(ListInstance::apply(gs, xs), List::empty());
    }

    #[test]
    fn test_apply_cartesian() {
        let add1: fn(i32) -> i32 = |x| x + 1;
        let mul2: fn(i32) -> i32 = |x| x * 2;
        let gs = List::from(vec![add1, mul2]);
        let xs = List::from(vec![1, 2]);
        let result = ListInstance::apply(gs, xs);
        let expected = List::from(vec![2, 3, 2, 4]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_applicative_identity_law() {
        let id = |x| x;
        let gs = List::singleton(arc(id));

        let xs = List::from(vec![1, 2, 3]);
        assert_eq!(ListInstance::apply(gs.clone(), xs.clone()), xs);

        let xs: List<i32> = List::empty();
        assert_eq!(ListInstance::apply(gs, xs), List::empty());
    }

    #[test]
    fn test_applicative_homomorphism_law() {
        let h = |x| x * 2;
        assert_eq!(
            ListInstance::apply(ListInstance::pure(arc(h)), ListInstance::pure(3)),
            ListInstance::pure(h(3)),
        );
    }

    #[test]
    fn test_applicative_interchange_law() {
        let add10: fn(i32) -> i32 = |x| x + 10;
        let mul2: fn(i32) -> i32 = |x| x * 2;
        let x = 5;

        let gs = List::from(vec![add10, mul2]);
        let lhs = ListInstance::apply(gs.clone(), ListInstance::pure(x));
        let rhs = ListInstance::apply(ListInstance::pure(arc(move |g: fn(i32) -> i32| g(x))), gs);
        assert_eq!(lhs, rhs);

        let gs: List<fn(i32) -> i32> = List::empty();
        let lhs = ListInstance::apply(gs.clone(), ListInstance::pure(x));
        let rhs = ListInstance::apply(ListInstance::pure(arc(move |g: fn(i32) -> i32| g(x))), gs);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_applicative_composition_law() {
        let add3 = WrappedFn::from(|x| x + 3);
        let mul2 = WrappedFn::from(|x| x * 2);
        let inc = WrappedFn::from(|x| x + 1);

        let xs = List::from(vec![1, 2]);
        let gs = List::from(vec![add3, mul2.clone()]);
        let hs = List::from(vec![inc, mul2]);

        let composed = ListInstance::apure(WrappedFn::curry(compose))
            .apply(gs.clone())
            .apply(hs.clone())
            .eval();
        let lhs = ListInstance::apply(composed, xs.clone());
        let rhs = ListInstance::apply(gs, ListInstance::apply(hs, xs));
        assert_eq!(lhs, rhs);
    }
}
