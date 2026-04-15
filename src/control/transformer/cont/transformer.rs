use std::marker::PhantomData;

use crate::base::computation::Computation;
use crate::base::function::{WrappedFn, WrappedTcFn};
use crate::base::value::{SimpleValue, Value};
use crate::control::context::ContextConstructor;
use crate::control::context::applicative::Applicative;
use crate::control::context::monad::Monad;
use crate::control::transformer::{MonadTrans, StackedMonadTrans, TransConstructor};

pub struct ContT<R, M, A>(WrappedTcFn<WrappedTcFn<A, M::Type<R>>, M::Type<R>>)
where
    R: Value,
    M: ContextConstructor;

impl<R, M, A> ContT<R, M, A>
where
    R: Value,
    M: ContextConstructor,
{
    pub fn new(inner: WrappedTcFn<WrappedTcFn<A, M::Type<R>>, M::Type<R>>) -> Self {
        Self(inner)
    }
}

impl<R, M, A> ContT<R, M, A>
where
    R: Value,
    M: ContextConstructor,
    A: Value,
{
    pub fn run_tr(
        trans: Self,
        continuation: WrappedTcFn<A, M::Type<R>>,
    ) -> Computation<M::Type<R>> {
        Computation::monadic(move || (trans.0)(continuation))
    }
}

impl<R, M> ContT<R, M, R>
where
    R: Value,
    M: Applicative,
{
    pub fn eval_tr(trans: Self) -> Computation<M::Type<R>> {
        Self::run_tr(
            trans,
            WrappedTcFn::from(move |x| Computation::immediate(M::pure(x))),
        )
    }
}

impl<R, M, A> Clone for ContT<R, M, A>
where
    R: Value,
    M: ContextConstructor,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<R, M, A> SimpleValue for ContT<R, M, A>
where
    R: Value,
    M: ContextConstructor,
    A: Value,
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ContTInstance<R>(PhantomData<R>);

impl<R> TransConstructor for ContTInstance<R>
where
    R: Value,
{
    type Type<M, A>
        = ContT<R, M, A>
    where
        M: Monad,
        A: Value;

    type Stacked<M>
        = StackedContTInstance<R, M>
    where
        M: Monad;
}

impl<R> MonadTrans for ContTInstance<R>
where
    R: Value,
{
    fn lift<M, A>(mx: M::Type<A>) -> Self::Type<M, A>
    where
        M: Monad,
        A: Value,
        Self::Stacked<M>: Monad<Type<A> = Self::Type<M, A>>,
    {
        ContT::new(WrappedTcFn::from(move |k: WrappedTcFn<A, M::Type<R>>| {
            let mx = mx.clone();
            Computation::lazy(move || M::bind(mx, WrappedFn::from(move |x| k(x).eval())))
        }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StackedContTInstance<R, M>(PhantomData<(R, M)>);

impl<R, M> ContextConstructor for StackedContTInstance<R, M>
where
    R: Value,
    M: ContextConstructor,
{
    type Type<A>
        = ContT<R, M, A>
    where
        A: Value;
}

impl<R, M> StackedMonadTrans for StackedContTInstance<R, M>
where
    R: Value,
    M: ContextConstructor,
{
    type Transformer = ContTInstance<R>;
}

#[cfg(test)]
mod tests {
    use crate::base::function::{WrappedFn, WrappedTcFn};
    use crate::control::context::monad::{Monad, MonadExt};
    use crate::control::transformer::cont::{Cont, ContInstance};

    #[test]
    fn test_cont_eval() {
        let c = Cont::cont(WrappedTcFn::from(|k: WrappedTcFn<i32, i32>| k(42)));
        assert_eq!(Cont::eval(c).eval(), 42);
    }

    #[test]
    fn test_cont_bind() {
        let c = ContInstance::ret(10).bind(WrappedFn::from(|x| ContInstance::ret(x * 2)));
        assert_eq!(Cont::eval(c).eval(), 20);
    }

    #[test]
    fn test_cont_shift() {
        let c = Cont::cont(WrappedTcFn::from(|k: WrappedTcFn<i32, i32>| {
            k(5).map(|r| r + 1)
        }));
        assert_eq!(Cont::eval(c).eval(), 6);
    }
}
