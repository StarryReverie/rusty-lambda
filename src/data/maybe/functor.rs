use std::borrow::Borrow;

use crate::base::value::{StaticConcurrent, Value};
use crate::control::functor::Functor;
use crate::data::maybe::{Maybe, MaybeInstance};

impl Functor for MaybeInstance {
    fn fmap<A, B, G, GI>(g: G, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: Borrow<GI> + Value,
        GI: Fn(A) -> B + StaticConcurrent,
    {
        match x {
            Maybe::Just(x) => Maybe::Just((g.borrow())(x)),
            Maybe::Nothing => Maybe::Nothing,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmap_just() {
        let g = |x| x + 1;
        let f = Maybe::Just(1);
        assert_eq!(MaybeInstance::fmap(g, f), Maybe::Just(2));
    }

    #[test]
    fn test_fmap_nothing() {
        let g = |x| x + 1;
        let f: Maybe<i32> = Maybe::Nothing;
        assert_eq!(MaybeInstance::fmap(g, f), Maybe::Nothing);
    }

    #[test]
    fn test_fmap_just_func() {
        fn g(h: impl Fn(i32) -> i32 + Value) -> impl Fn(i32) -> i32 + Value {
            move |x| h(x) * 2
        }
        let x = Maybe::Just(|x| x + 1);
        MaybeInstance::fmap(g, x);
    }

    #[test]
    fn test_functor_identity_law() {
        let id = |x| x;

        let f = Maybe::Just(42);
        assert_eq!(MaybeInstance::fmap(id, f), Maybe::Just(42));

        let f: Maybe<i32> = Maybe::Nothing;
        assert_eq!(MaybeInstance::fmap(id, f), Maybe::Nothing);
    }

    #[test]
    fn test_functor_composition_law() {
        let h = |x| (x as i64) * 2;
        let g = |x| x + 3;
        let composed = move |x| g(h(x));

        let f = Maybe::Just(4i32);
        let lhs = MaybeInstance::fmap(composed, f);
        let rhs = MaybeInstance::fmap(g, MaybeInstance::fmap(h, Maybe::Just(4i32)));
        assert_eq!(lhs, Maybe::Just(11i64));
        assert_eq!(lhs, rhs);

        let f: Maybe<i32> = Maybe::Nothing;
        let lhs = MaybeInstance::fmap(composed, f);
        let rhs = MaybeInstance::fmap(g, MaybeInstance::fmap(h, Maybe::Nothing));
        assert_eq!(lhs, Maybe::Nothing);
        assert_eq!(lhs, rhs);
    }
}
