use std::marker::PhantomData;

use crate::base::function::WrappedFn;
use crate::base::value::{SimpleValue, Value};
use crate::control::context::ContextConstructor;

pub struct State<S, A>(pub WrappedFn<S, (A, S)>);

impl<S, A> State<S, A>
where
    S: Value,
    A: Value,
{
    pub fn run(run: &Self, s: S) -> (A, S) {
        (run.0)(s)
    }

    pub fn eval(run: &Self, s: S) -> A {
        Self::run(run, s).0
    }

    pub fn exec(run: &Self, s: S) -> S {
        Self::run(run, s).1
    }
}

impl<S, A, F> From<F> for State<S, A>
where
    F: Into<WrappedFn<S, (A, S)>>,
{
    fn from(func: F) -> Self {
        Self(func.into())
    }
}

impl<S, A> Clone for State<S, A> {
    fn clone(&self) -> Self {
        State(self.0.clone())
    }
}

impl<S, A> SimpleValue for State<S, A>
where
    S: Value,
    A: Value,
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StateInstance<S>(PhantomData<S>);

impl<S> ContextConstructor for StateInstance<S>
where
    S: Value,
{
    type Type<A>
        = State<S, A>
    where
        A: Value;
}

impl<S> State<S, S>
where
    S: Value,
{
    pub fn get() -> Self {
        State::from(|s: S| (s.clone(), s))
    }
}

impl<S> State<S, ()>
where
    S: Value,
{
    pub fn put(s: S) -> Self {
        State::from(move |_| ((), s.clone()))
    }

    pub fn modify<G>(g: G) -> Self
    where
        G: Into<WrappedFn<S, S>>,
    {
        let g = g.into();
        State::from(move |s| ((), g(s)))
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::WrappedFn;
    use crate::control::context::monad::state::StateInstance;
    use crate::control::context::monad::{Monad, MonadExt};
    use crate::data::list::List;
    use crate::data::maybe::Maybe;

    use super::*;

    #[test]
    fn test_get() {
        let state = State::run(&State::get(), 42);
        assert_eq!(state, (42, 42));
    }

    #[test]
    fn test_put() {
        let state = State::run(&State::put(99), 0);
        assert_eq!(state, ((), 99));
    }

    #[test]
    fn test_stack_push_pop() {
        let push = |x| WrappedFn::from(move |_| State::from(move |s| ((), List::cons(x, s))));
        let pop = || {
            State::from(|s: List<i32>| match s.clone().decompose() {
                Maybe::Just((head, tail)) => (head, tail),
                Maybe::Nothing => (0, s),
            })
        };

        let state = State::from(|s| ((), s))
            .bind(push(1))
            .bind(push(2))
            .bind(push(3))
            .bind(WrappedFn::from(move |_| pop()))
            .bind(WrappedFn::from(move |_| pop()));

        let (top, stack) = State::run(&state, List::empty());
        assert_eq!(top, 2);
        assert_eq!(stack, List::singleton(1));
    }

    #[test]
    fn test_eval_vs_exec() {
        let m =
            StateInstance::ret(1).bind(WrappedFn::from(|x| State::from(move |s| (x + s, s * 2))));
        assert_eq!(State::eval(&m, 3), 4);
        assert_eq!(State::exec(&m, 3), 6);
    }
}
