#![expect(clippy::type_complexity)]

use std::marker::PhantomData;

use crate::base::function::WrappedFn;
use crate::base::lazy::Thunk;
use crate::base::value::{SimpleValue, Value};
use crate::control::context::ContextConstructor;
use crate::control::context::applicative::Applicative;
use crate::control::context::monad::Monad;
use crate::data::maybe::Maybe;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LogicT<M, A>(Thunk<M::Type<Maybe<(A, LogicT<M, A>)>>>)
where
    M: ContextConstructor,
    A: Value;

impl<M, A> LogicT<M, A>
where
    M: ContextConstructor,
    A: Value,
{
    pub fn new(thunk: Thunk<M::Type<Maybe<(A, LogicT<M, A>)>>>) -> Self {
        Self(thunk)
    }

    pub fn decompose(&self) -> M::Type<Maybe<(A, Self)>> {
        self.0.force().clone()
    }
}

impl<M, A> LogicT<M, A>
where
    M: Applicative,
    A: Value,
{
    pub fn empty() -> Self {
        Self::new(Thunk::immediate(M::pure(Maybe::Nothing)))
    }

    pub fn cons(head: A, tail: Self) -> Self {
        Self::new(Thunk::lazy(move || M::pure(Maybe::Just((head, tail)))))
    }

    pub fn singleton(value: A) -> Self {
        Self::cons(value, Self::empty())
    }
}

impl<M, A> LogicT<M, A>
where
    M: Monad,
    A: Value,
{
    pub fn append(self, other: Self) -> Self {
        Self::new(Thunk::lazy(move || {
            M::bind(
                self.decompose(),
                WrappedFn::from(move |node: Maybe<(A, Self)>| match node {
                    Maybe::Just((head, tail)) => {
                        let tail = tail.append(other.clone());
                        M::pure(Maybe::Just((head, tail)))
                    }
                    Maybe::Nothing => other.decompose(),
                }),
            )
        }))
    }
}

impl<M, A> SimpleValue for LogicT<M, A>
where
    M: ContextConstructor,
    A: Value,
{
}

impl<M, A> Clone for LogicT<M, A>
where
    M: ContextConstructor,
    A: Value,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LogicTInstance;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StackedLogicTInstance<M>(PhantomData<M>);

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    use crate::control::structure::functor::identity::{Identity, IdentityInstance};
    use crate::data::maybe::Maybe;

    use super::*;

    type Logic<A> = LogicT<IdentityInstance, A>;

    fn run_logic<A: Value>(l: &Logic<A>) -> Vec<A> {
        let mut results = Vec::new();
        let mut cur = l.clone();
        loop {
            let node = Identity::run(cur.decompose());
            match node {
                Maybe::Just((a, tail)) => {
                    results.push(a);
                    cur = tail;
                }
                Maybe::Nothing => break,
            }
        }
        results
    }

    #[test]
    fn test_creation() {
        let l: Logic<i32> = LogicT::empty();
        assert_eq!(run_logic(&l), Vec::<i32>::new());

        let l = LogicT::cons(1, LogicT::cons(2, LogicT::cons(3, LogicT::empty())));
        assert_eq!(run_logic(&l), vec![1, 2, 3]);

        let l = LogicT::singleton(42);
        assert_eq!(run_logic(&l), vec![42]);
    }

    #[test]
    fn test_append() {
        let a = LogicT::cons(1, LogicT::cons(2, LogicT::empty()));
        let b = LogicT::cons(3, LogicT::cons(4, LogicT::empty()));
        let l = a.append(b);
        assert_eq!(run_logic(&l), vec![1, 2, 3, 4]);

        let a = LogicT::empty();
        let b = LogicT::cons(1, LogicT::cons(2, LogicT::empty()));
        let l = a.append(b);
        assert_eq!(run_logic(&l), vec![1, 2]);

        let a = LogicT::cons(1, LogicT::cons(2, LogicT::empty()));
        let b = LogicT::empty();
        let l = a.append(b);
        assert_eq!(run_logic(&l), vec![1, 2]);

        let a: Logic<i32> = LogicT::empty();
        let b: Logic<i32> = LogicT::empty();
        let l = a.append(b);
        assert_eq!(run_logic(&l), Vec::<i32>::new());
    }

    #[test]
    fn test_lazy_cons_not_evaluated() {
        let evaluated = Arc::new(AtomicBool::new(false));
        let c = evaluated.clone();
        let l = Logic::cons(
            1,
            LogicT::new(Thunk::lazy(move || {
                let _ = c.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst);
                Identity(Maybe::Just((99, LogicT::empty())))
            })),
        );
        let _ = l.decompose();
        assert_eq!(evaluated.load(Ordering::SeqCst), false);
        assert_eq!(run_logic(&l), vec![1, 99]);
        assert_eq!(evaluated.load(Ordering::SeqCst), true);
    }
}
