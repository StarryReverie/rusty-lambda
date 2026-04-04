use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::structure::functor::Functor;
use crate::data::maybe::{Maybe, MaybeInstance};

impl Functor for MaybeInstance {
    fn fmap<A, B, G>(g: G, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        match x {
            Maybe::Just(x) => Maybe::Just(g.view().call(x)),
            Maybe::Nothing => Maybe::Nothing,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::WrappedFn;
    use crate::base::value::arc;

    use super::*;

    #[test]
    fn test_fmap_just() {
        let g = |x| x + 1;
        let f = Maybe::Just(1);
        assert_eq!(MaybeInstance::fmap(arc(g), f), Maybe::Just(2));
    }

    #[test]
    fn test_fmap_nothing() {
        let g = |x| x + 1;
        let f: Maybe<i32> = Maybe::Nothing;
        assert_eq!(MaybeInstance::fmap(arc(g), f), Maybe::Nothing);
    }

    #[test]
    fn test_fmap_just_func() {
        fn g(h: WrappedFn<i32, i32>) -> WrappedFn<i32, i32> {
            WrappedFn::from(move |x| h(x) * 2)
        }

        let f = Maybe::Just(WrappedFn::from(|x: i32| x + 1));
        match MaybeInstance::fmap(arc(g), f) {
            Maybe::Just(h) => assert_eq!(h(1), 4),
            Maybe::Nothing => unreachable!(),
        }
    }

    #[test]
    fn test_functor_identity_law() {
        let id = |x| x;

        let f = Maybe::Just(42);
        assert_eq!(MaybeInstance::fmap(arc(id), f), Maybe::Just(42));

        let f: Maybe<i32> = Maybe::Nothing;
        assert_eq!(MaybeInstance::fmap(arc(id), f), Maybe::Nothing,);
    }

    #[test]
    fn test_functor_composition_law() {
        let h = |x| (x as i64) * 2;
        let g = |x| x + 3;
        let composed = g.compose(h);

        let f = Maybe::Just(4i32);
        let lhs = MaybeInstance::fmap(composed.clone(), f);
        let rhs = MaybeInstance::fmap(arc(g), MaybeInstance::fmap(arc(h), Maybe::Just(4i32)));
        assert_eq!(lhs, Maybe::Just(11i64));
        assert_eq!(lhs, rhs);

        let f: Maybe<i32> = Maybe::Nothing;
        let lhs = MaybeInstance::fmap(composed, f);
        let rhs = MaybeInstance::fmap(arc(g), MaybeInstance::fmap(arc(h), Maybe::Nothing));
        assert_eq!(lhs, Maybe::Nothing);
        assert_eq!(lhs, rhs);
    }
}
