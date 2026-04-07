use crate::base::function::ConcurrentFn;
use crate::base::value::{StaticConcurrent, Value};
use crate::control::context::monad::state::{State, StateInstance};
use crate::control::structure::functor::{Functor, FunctorExt};

impl<S> Functor for StateInstance<S>
where
    S: Value,
{
    fn fmap<A, B, G>(g: G, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        State::from(move |s| {
            let (a, s) = State::run(&x, s);
            (g.view().call(a), s)
        })
    }
}

impl<S, A> FunctorExt for State<S, A>
where
    S: Value,
    A: StaticConcurrent,
{
    type Wrapped = A;
    type Instance = StateInstance<S>;
}

#[cfg(test)]
mod tests {
    use crate::{base::function::WrappedFn, control::structure::functor::fmap};

    use super::*;

    #[test]
    fn test_fmap() {
        let state = State::from(|s| (s * 2, s));
        let state = fmap(WrappedFn::from(|x| x + 1), state);
        assert_eq!(State::run(&state, 3), (7, 3));
    }

    #[test]
    fn test_functor_identity_law() {
        let state = State::from(|s| (s + 10, s));
        let state = fmap(WrappedFn::from(|x| x), state);
        assert_eq!(State::run(&state, 5), (15, 5));
    }

    #[test]
    fn test_functor_composition_law() {
        let h = WrappedFn::from(|x| x * 2);
        let g = WrappedFn::from(|x| x + 3);
        let composed = g.clone().compose(h.clone());

        let state = State::from(|s| (s, s));
        let lhs = fmap(composed, state.clone());
        let rhs = fmap(g, fmap(h, state));
        assert_eq!(State::run(&lhs, 4), (11, 4));
        assert_eq!(State::run(&rhs, 4), (11, 4));
    }
}
