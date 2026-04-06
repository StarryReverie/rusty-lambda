use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

use crate::base::function::{ConcurrentFn, constv};
use crate::base::hkt::TypeConstructor1;
use crate::base::value::{StaticConcurrent, Value};
use crate::control::context::applicative::{Applicative, ApplicativeExt};
use crate::control::context::monad::{Monad, MonadExt};
use crate::control::structure::functor::Functor;

pub struct WrappedFn<T, R>(Arc<dyn Fn(T) -> R + Send + Sync + 'static>);

impl<T, R, F> From<F> for WrappedFn<T, R>
where
    F: Fn(T) -> R + StaticConcurrent,
{
    fn from(func: F) -> Self {
        Self(Arc::new(func))
    }
}

impl<T, R> ConcurrentFn<T> for WrappedFn<T, R> {
    type Output = R;

    fn call(&self, argument: T) -> Self::Output {
        (self.0)(argument)
    }
}

impl<T, R> Value for WrappedFn<T, R>
where
    T: StaticConcurrent,
    R: StaticConcurrent,
{
    type View<'a>
        = &'a <Self as Deref>::Target
    where
        Self: 'a;

    fn view(&self) -> Self::View<'_> {
        &**self
    }
}

impl<T, R> Deref for WrappedFn<T, R> {
    type Target = dyn Fn(T) -> R + Send + Sync;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<T, R> Clone for WrappedFn<T, R> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct WrappedFnInstance<T>(PhantomData<T>);

impl<E> TypeConstructor1 for WrappedFnInstance<E>
where
    E: StaticConcurrent,
{
    type Type<A>
        = WrappedFn<E, A>
    where
        A: StaticConcurrent;
}

impl<E> Functor for WrappedFnInstance<E>
where
    E: Value,
{
    fn fmap<A, B, G>(g: G, f: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        WrappedFn::from(move |x| g.view().call(f.call(x)))
    }
}

impl<E> Applicative for WrappedFnInstance<E>
where
    E: Value,
{
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        constv(x)
    }

    fn apply<A, B, G>(g: Self::Type<G>, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        WrappedFn::from(move |e: E| {
            let g = g(e.clone());
            let x = x(e);
            g.view().call(x)
        })
    }
}

impl<E> Monad for WrappedFnInstance<E>
where
    E: Value,
{
    fn bind<A, B, G>(x: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        WrappedFn::from(move |e: E| {
            let x = x(e.clone());
            let g = g.view().call(x);
            g(e)
        })
    }
}

impl<E, A> ApplicativeExt for WrappedFn<E, A>
where
    E: Value,
    A: StaticConcurrent,
{
    type Wrapped = A;
    type Instance = WrappedFnInstance<E>;
}

impl<E, A> MonadExt for WrappedFn<E, A>
where
    E: Value,
    A: StaticConcurrent,
{
    type Wrapped = A;
    type Instance = WrappedFnInstance<E>;
}

#[cfg(test)]
mod tests {
    use crate::base::function::{Curry, WrappedFn, compose};

    use super::*;

    #[test]
    fn test_wrapped_fn_ref() {
        let inc = |x: &mut i32| {
            *x += 1;
        };
        let mut x = 1;
        {
            let inc = WrappedFn::from(inc);
            inc.call(&mut x);
        }
        assert_eq!(x, 2);
    }

    #[test]
    fn test_functor_identity_law() {
        let id = WrappedFn::from(|x| x);
        let reader = WrappedFn::from(|e| e + 1);
        let result = WrappedFnInstance::fmap(id, reader);
        assert_eq!(result(5), 6);
        assert_eq!(result(0), 1);
    }

    #[test]
    fn test_applicative_identity_law() {
        let id = WrappedFn::from(|x| x);
        let reader = WrappedFnInstance::pure(42);
        let result = WrappedFnInstance::pure(id).apply(reader);
        assert_eq!(result(99), 42);
    }

    #[test]
    fn test_applicative_homomorphism_law() {
        let h = WrappedFn::from(|x| x * 2);
        let lhs = WrappedFnInstance::pure(h.clone()).apply(WrappedFnInstance::pure(3));
        let rhs = WrappedFnInstance::pure(h(3));
        assert_eq!(lhs(99), rhs(99));
    }

    #[test]
    fn test_applicative_interchange_law() {
        let h = WrappedFn::curry(|e, x| x + e);
        let x = 5;

        let lhs = h.clone().apply(WrappedFnInstance::pure(x));
        let rhs =
            WrappedFnInstance::pure(WrappedFn::from(move |g: WrappedFn<i32, i32>| g(x))).apply(h);
        assert_eq!(lhs(3), rhs(3));
        assert_eq!(lhs(10), rhs(10));
    }

    #[test]
    fn test_applicative_composition_law() {
        let g = WrappedFn::curry(|_e, x| x * 2);
        let h = WrappedFn::curry(|_e, x| x + 3);
        let composed = WrappedFnInstance::pure(WrappedFn::curry(compose))
            .apply(g.clone())
            .apply(h.clone());

        let x = WrappedFnInstance::pure(4);
        let lhs = composed.apply(x.clone());
        let rhs = g.apply(h.apply(x));
        assert_eq!(lhs(99), rhs(99));
    }

    #[test]
    fn test_monad_left_identity_law() {
        let g = WrappedFn::curry(|x, e| x + e);
        let res = WrappedFnInstance::ret(3).bind(g.clone());
        assert_eq!(res(10), g(3)(10));
    }

    #[test]
    fn test_monad_right_identity_law() {
        let m = WrappedFn::from(|e: i32| e * 2);
        let res = m.bind(WrappedFn::from(WrappedFnInstance::ret));
        assert_eq!(res(5), 10);
    }

    #[test]
    fn test_monad_associativity_law() {
        let g = WrappedFn::curry(|x, _e| x + 1);
        let h = WrappedFn::curry(|x, _e| x * 2);
        let lhs = WrappedFnInstance::ret(3).bind(g.clone()).bind(h.clone());
        let rhs = WrappedFnInstance::ret(3).bind(WrappedFn::from(move |x| g(x).bind(h.clone())));
        assert_eq!(lhs(99), rhs(99));
    }
}
