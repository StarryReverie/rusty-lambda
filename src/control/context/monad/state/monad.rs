use crate::base::function::ConcurrentFn;
use crate::base::value::{StaticConcurrent, Value};
use crate::control::context::monad::state::{State, StateInstance};
use crate::control::context::monad::{Monad, MonadExt};

impl<S> Monad for StateInstance<S>
where
    S: Value,
{
    fn bind<A, B, G>(x: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        State::from(move |s| {
            let (a, s) = State::run(&x, s);
            let r = g.view().call(a);
            State::run(&r, s)
        })
    }
}

impl<S, A> MonadExt for State<S, A>
where
    S: Value,
    A: StaticConcurrent,
{
    type Wrapped = A;
    type Instance = StateInstance<S>;
}

#[cfg(test)]
mod tests {
    use crate::base::function::WrappedFn;
    use crate::control::context::monad::state::wrapper::State;

    use super::*;

    #[test]
    fn test_bind() {
        let m = State::from(|s| (s * 2, s + 1));
        let state = StateInstance::mchain(m)
            .bind(WrappedFn::from(|x| State::from(move |s: i32| (x + s, s))))
            .eval();
        assert_eq!(State::run(&state, 3), (10, 4));
    }

    #[test]
    fn test_monad_left_identity_law() {
        let g = WrappedFn::from(|x| State::from(move |s| (x * 2, s + 1)));
        let lhs = StateInstance::mreturn(3).bind(g.clone()).eval();
        let rhs = g(3);
        assert_eq!(State::run(&lhs, 10), State::run(&rhs, 10));
    }

    #[test]
    fn test_monad_right_identity_law() {
        let m = StateInstance::mchain(State::from(|s| (s + 5, s * 2)))
            .bind(WrappedFn::from(|x| StateInstance::ret(x)))
            .eval();
        assert_eq!(State::run(&m, 3), (8, 6));
    }

    #[test]
    fn test_monad_associativity_law() {
        let g = WrappedFn::from(|x| State::from(move |s| (x + 1, s + 10)));
        let h = WrappedFn::from(|x| State::from(move |s| (x * 2, s * 3)));

        let m = State(WrappedFn::from(|s| (s, s)));
        let lhs = StateInstance::mchain(m.clone())
            .bind(g.clone())
            .bind(h.clone())
            .eval();
        let rhs = StateInstance::mchain(m)
            .bind(WrappedFn::from(move |x| {
                StateInstance::mchain(g(x)).bind(h.clone()).eval()
            }))
            .eval();
        assert_eq!(State::run(&lhs, 5), State::run(&rhs, 5));
        assert_eq!(State::run(&lhs, 5), ((5 + 1) * 2, (5 + 10) * 3));
    }

    #[test]
    fn test_chained_bind() {
        let m = StateInstance::mreturn(1)
            .bind(WrappedFn::from(|x| State::from(move |s| (x + s, s + 1))))
            .bind(WrappedFn::from(|x| State::from(move |s| (x * 3, s * 2))))
            .eval();
        assert_eq!(State::run(&m, 10), (33, 22));
    }
}
