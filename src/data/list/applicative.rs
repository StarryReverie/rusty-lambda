use std::borrow::Borrow;

use crate::base::value::{StaticConcurrent, Value};
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

    fn apply<A, B, G, GI>(gs: Self::Type<G>, xs: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: Borrow<GI> + Value,
        GI: Fn(A) -> B + StaticConcurrent,
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
    use super::*;

    #[test]
    fn test_pure() {
        assert_eq!(ListInstance::pure(42), List::singleton(42));
    }

    #[test]
    fn test_apply_nil_gs() {
        let gs: List<fn(i32) -> i32> = List::empty();
        let xs = List::cons(1, List::cons(2, List::empty()));
        assert_eq!(ListInstance::apply(gs, xs), List::empty());
    }

    #[test]
    fn test_apply_nil_xs() {
        let gs = List::singleton(|x: i32| x + 1);
        let xs: List<i32> = List::empty();
        assert_eq!(ListInstance::apply(gs, xs), List::empty());
    }

    #[test]
    fn test_apply_cartesian() {
        let add1: fn(i32) -> i32 = |x| x + 1;
        let mul2: fn(i32) -> i32 = |x| x * 2;
        let gs = List::cons(add1, List::cons(mul2, List::empty()));
        let xs = List::cons(1, List::cons(2, List::empty()));
        let result = ListInstance::apply(gs, xs);
        let expected = List::cons(
            2,
            List::cons(3, List::cons(2, List::cons(4, List::empty()))),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_applicative_identity_law() {
        let id = |x| x;

        let xs = List::cons(1, List::cons(2, List::cons(3, List::empty())));
        assert_eq!(ListInstance::apply(List::singleton(id), xs.clone()), xs);

        let xs: List<i32> = List::empty();
        assert_eq!(ListInstance::apply(List::singleton(id), xs), List::empty());
    }

    #[test]
    fn test_applicative_homomorphism_law() {
        let h = |x| x * 2;
        assert_eq!(
            ListInstance::apply(ListInstance::pure(h), ListInstance::pure(3)),
            ListInstance::pure(h(3))
        );
    }

    #[test]
    fn test_applicative_interchange_law() {
        let add10: fn(i32) -> i32 = |x| x + 10;
        let mul2: fn(i32) -> i32 = |x| x * 2;
        let x = 5;

        let gs = List::cons(add10, List::cons(mul2, List::empty()));
        let lhs = ListInstance::apply(gs.clone(), ListInstance::pure(x));
        let rhs = ListInstance::apply(ListInstance::pure(move |g: fn(i32) -> i32| g(x)), gs);
        assert_eq!(lhs, rhs);

        let gs: List<fn(i32) -> i32> = List::empty();
        let lhs = ListInstance::apply(gs.clone(), ListInstance::pure(x));
        let rhs = ListInstance::apply(ListInstance::pure(move |g: fn(i32) -> i32| g(x)), gs);
        assert_eq!(lhs, rhs);
    }

    // #[test]
    // fn test_applicative_composition_law() {
    //     let compose =
    //         |g: fn(i32) -> i32| -> Arc<dyn Fn(fn(i32) -> i32) -> Arc<dyn Fn(i32) -> i32> + Send + Sync>
    //     {
    //         Arc::new(move |h: fn(i32) -> i32| Arc::new(move |x: i32| g(h(x))))
    //     };

    //     let add3: fn(i32) -> i32 = |x| x + 3;
    //     let mul2: fn(i32) -> i32 = |x| x * 2;
    //     let inc: fn(i32) -> i32 = |x| x + 1;

    //     let xs = List::cons(1, List::cons(2, List::empty()));
    //     let fs = List::cons(add3, List::cons(mul2, List::empty()));
    //     let hs = List::cons(inc, List::cons(mul2, List::empty()));

    //     let composed = ListInstance::apply(
    //         ListInstance::apply(ListInstance::pure(compose), fs.clone()),
    //         hs.clone(),
    //     );
    //     let lhs = ListInstance::apply(composed, xs.clone());
    //     let rhs = ListInstance::apply(fs, ListInstance::apply(hs, xs));
    //     assert_eq!(lhs, rhs);
    // }
}
