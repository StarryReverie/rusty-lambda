use crate::base::function::{ConcurrentFn, WrappedFn};
use crate::base::value::Value;
use crate::control::context::applicative::{Applicative, ApplicativeExt};
use crate::control::context::monad::{Monad, MonadExt};
use crate::control::structure::functor::{Functor, FunctorExt};
use crate::control::transformer::state::StateT;
use crate::control::transformer::state::transformer::StackedStateTInstance;

impl<S, M> Functor for StackedStateTInstance<S, M>
where
    S: Value,
    M: Functor,
{
    fn fmap<A, B, G>(g: G, fx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        StateT::new(WrappedFn::from(move |state| {
            let mx = StateT::run_tr(&fx, state);
            let g = g.clone();
            M::fmap(WrappedFn::from(move |(x, s)| (g.view().call(x), s)), mx)
        }))
    }
}

impl<S, M, A> FunctorExt for StateT<S, M, A>
where
    S: Value,
    M: Functor,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedStateTInstance<S, M>;
}

impl<S, M> Applicative for StackedStateTInstance<S, M>
where
    S: Value,
    M: Monad,
{
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        StateT::new(WrappedFn::from(move |s| M::pure((x.clone(), s))))
    }

    fn apply<A, B, G>(smg: Self::Type<G>, smx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        StateT::new(WrappedFn::from(move |s: S| {
            let mg = StateT::run_tr(&smg, s);
            let smx = smx.clone();
            M::bind(
                mg,
                WrappedFn::from(move |(g, s): (G, S)| {
                    let mx = StateT::run_tr(&smx, s);
                    M::fmap(
                        WrappedFn::from(move |(x, s): (A, S)| (g.view().call(x), s)),
                        mx,
                    )
                }),
            )
        }))
    }
}

impl<S, M, A> ApplicativeExt for StateT<S, M, A>
where
    S: Value,
    M: Monad,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedStateTInstance<S, M>;
}

impl<S, M> Monad for StackedStateTInstance<S, M>
where
    S: Value,
    M: Monad,
{
    fn bind<A, B, G>(smx: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        StateT::new(WrappedFn::from(move |s: S| {
            let mx = StateT::run_tr(&smx, s);
            let g = g.clone();
            M::bind(
                mx,
                WrappedFn::from(move |(x, s): (A, S)| {
                    let smy = g.view().call(x);
                    StateT::run_tr(smy, s)
                }),
            )
        }))
    }
}

impl<S, M, A> MonadExt for StateT<S, M, A>
where
    S: Value,
    M: Monad,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedStateTInstance<S, M>;
}

#[cfg(test)]
mod tests {
    use crate::base::function::{ConcurrentFn, Curry, WrappedFn, compose};
    use crate::control::context::applicative::{Applicative, ApplicativeExt};
    use crate::control::context::monad::{Monad, MonadExt};
    use crate::control::structure::functor::fmap;
    use crate::control::transformer::state::{State, StateInstance};

    #[test]
    fn test_functor_identity_law() {
        let state = State::from(|s| (s + 10, s));
        let state = fmap(WrappedFn::from(|x| x), state);
        assert_eq!(State::run(state, 5), (15, 5));
    }

    #[test]
    fn test_functor_composition_law() {
        let h = WrappedFn::from(|x| x * 2);
        let g = WrappedFn::from(|x| x + 3);
        let composed = g.clone().compose(h.clone());

        let state = State::from(|s| (s, s));
        let lhs = fmap(composed, state.clone());
        let rhs = fmap(g, fmap(h, state));
        assert_eq!(State::run(lhs, 4), (11, 4));
        assert_eq!(State::run(rhs, 4), (11, 4));
    }

    #[test]
    fn test_applicative_identity_law() {
        let state = StateInstance::pure(WrappedFn::from(|x| x)).apply(State::from(|s| (s + 10, s)));
        assert_eq!(State::run(state, 5), (15, 5));
    }

    #[test]
    fn test_applicative_homomorphism_law() {
        let h = WrappedFn::from(|x| x * 2);
        let lhs = StateInstance::pure(h.clone()).apply(StateInstance::pure(3));
        let rhs = StateInstance::pure(h(3));
        assert_eq!(State::run(lhs, 99), State::run(rhs, 99));
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

    #[test]
    fn test_monad_left_identity_law() {
        let g = WrappedFn::from(|x| State::from(move |s| (x * 2, s + 1)));
        let lhs = StateInstance::ret(3).bind(g.clone());
        let rhs = g(3);
        assert_eq!(State::run(lhs, 10), State::run(rhs, 10));
    }

    #[test]
    fn test_monad_right_identity_law() {
        let m = State::from(|s| (s + 5, s * 2)).bind(WrappedFn::from(|x| StateInstance::ret(x)));
        assert_eq!(State::run(m, 3), (8, 6));
    }

    #[test]
    fn test_monad_associativity_law() {
        let g = WrappedFn::from(|x| State::from(move |s| (x + 1, s + 10)));
        let h = WrappedFn::from(|x| State::from(move |s| (x * 2, s * 3)));

        let m = State::from(|s| (s, s));
        let lhs = m.clone().bind(g.clone()).bind(h.clone());
        let rhs = m.bind(WrappedFn::from(move |x| g(x).bind(h.clone())));
        assert_eq!(State::run(&lhs, 5), State::run(&rhs, 5));
        assert_eq!(State::run(&lhs, 5), ((5 + 1) * 2, (5 + 10) * 3));
    }
}
