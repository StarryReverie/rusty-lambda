use crate::base::value::Value;
use crate::control::structure::functor::identity::IdentityInstance;
use crate::control::transformer::state::{StackedStateTInstance, StateT};

pub type State<S, A> = StateT<S, IdentityInstance, A>;
pub type StateInstance<S> = StackedStateTInstance<S, IdentityInstance>;

impl<S, A> State<S, A>
where
    S: Value,
    A: Value,
{
    pub fn run(mx: Self, state: S) -> (A, S) {
        Self::run_tr(mx, state).0
    }

    pub fn eval(mx: Self, state: S) -> A {
        Self::run(mx, state).0
    }

    pub fn exec(mx: Self, state: S) -> S {
        Self::run(mx, state).1
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::WrappedFn;
    use crate::control::context::monad::{Monad, MonadExt};
    use crate::control::transformer::state::MonadState;
    use crate::data::list::List;
    use crate::data::maybe::Maybe;

    use super::*;

    #[test]
    fn test_get() {
        let state = StateInstance::get();
        assert_eq!(State::run(state, 42), (42, 42));
    }

    #[test]
    fn test_put() {
        let state = StateInstance::put(99);
        assert_eq!(State::run(state, 0), ((), 99));
    }

    #[test]
    fn test_stack_push_pop() {
        let push = |x| WrappedFn::from(move |_| State::from(move |s| ((), List::cons(x, s))));
        let pop = || {
            State::from(|s: List<i32>| match s.decompose() {
                Maybe::Just((head, tail)) => (head, tail),
                Maybe::Nothing => (0, List::empty()),
            })
        };

        let state = State::from(|s| ((), s))
            .bind(push(1))
            .bind(push(2))
            .bind(push(3))
            .bind(WrappedFn::from(move |_| pop()))
            .bind(WrappedFn::from(move |_| pop()));

        let (top, stack) = State::run(state, List::empty());
        assert_eq!(top, 2);
        assert_eq!(stack, List::singleton(1));
    }

    #[test]
    fn test_eval_vs_exec() {
        let m =
            StateInstance::ret(1).bind(WrappedFn::from(|x| State::from(move |s| (x + s, s * 2))));
        assert_eq!(State::eval(m.clone(), 3), 4);
        assert_eq!(State::exec(m, 3), 6);
    }
}
