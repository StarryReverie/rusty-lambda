use crate::base::function::{ConcurrentFn, WrappedFn};
use crate::base::value::Value;
use crate::control::context::applicative::{Applicative, ApplicativeExt};
use crate::control::context::monad::state::{State, StateInstance};

impl<S> Applicative for StateInstance<S>
where
    S: Value,
{
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        State(WrappedFn::from(move |s| (x.clone(), s)))
    }

    fn apply<A, B, G>(g: Self::Type<G>, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        State(WrappedFn::from(move |s| {
            let (g, s) = State::run(&g, s);
            let (x, s) = State::run(&x, s);
            (g.view().call(x), s)
        }))
    }
}

impl<S, A> ApplicativeExt for State<S, A>
where
    S: Value,
    A: Value,
{
    type Wrapped = A;
    type Instance = StateInstance<S>;
}

#[cfg(test)]
mod tests {
    use crate::base::function::{Curry, WrappedFn, compose};

    use super::*;

    #[test]
    fn test_pure() {
        let state = StateInstance::pure(42);
        assert_eq!(State::run(&state, 99), (42, 99));
    }

    #[test]
    fn test_apply() {
        let g = State::from(|s| (WrappedFn::from(move |x| x + s), s));
        let x = State::from(|s| (10, s));
        let state = g.apply(x);
        assert_eq!(State::run(&state, 5), (15, 5));
    }

    #[test]
    fn test_applicative_identity_law() {
        let state = StateInstance::pure(WrappedFn::from(|x| x)).apply(State::from(|s| (s + 10, s)));
        assert_eq!(State::run(&state, 5), (15, 5));
    }

    #[test]
    fn test_applicative_homomorphism_law() {
        let h = WrappedFn::from(|x| x * 2);
        let lhs = StateInstance::pure(h.clone()).apply(StateInstance::pure(3));
        let rhs = StateInstance::pure(h(3));
        assert_eq!(State::run(&lhs, 99), State::run(&rhs, 99));
    }

    #[test]
    fn test_applicative_interchange_law() {
        let h = State::from(|s| (WrappedFn::from(move |x| x + s), s));
        let x = 5;

        let lhs = h.clone().apply(StateInstance::pure(x));
        let rhs = StateInstance::pure(WrappedFn::from(move |g: WrappedFn<i32, i32>| g(x))).apply(h);
        assert_eq!(State::run(&lhs, 3), State::run(&rhs, 3));
        assert_eq!(State::run(&lhs, 10), State::run(&rhs, 10));
    }

    #[test]
    fn test_applicative_composition_law() {
        let g = State::from(|s| (WrappedFn::from(move |x| x * s), s));
        let h = State::from(|s| (WrappedFn::from(move |x| x + s), s));
        let composed = StateInstance::pure(WrappedFn::curry(compose))
            .apply(g.clone())
            .apply(h.clone());

        let x = StateInstance::pure(4);
        let lhs = composed.apply(x.clone());
        let rhs = g.apply(h.apply(x));
        assert_eq!(State::run(&lhs, 3), State::run(&rhs, 3));
        assert_eq!(State::run(&lhs, 10), State::run(&rhs, 10));
    }
}
