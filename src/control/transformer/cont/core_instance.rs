use crate::base::computation::Computation;
use crate::base::function::{ConcurrentFn, WrappedTcFn};
use crate::base::value::Value;
use crate::control::context::ContextConstructor;
use crate::control::context::applicative::{Applicative, ApplicativeExt};
use crate::control::context::monad::{Monad, MonadExt};
use crate::control::structure::functor::{Functor, FunctorExt};
use crate::control::transformer::cont::{ContT, StackedContTInstance};

impl<R, M> Functor for StackedContTInstance<R, M>
where
    R: Value,
    M: ContextConstructor,
{
    #[rustfmt::skip]
    fn fmap<A, B, G>(g: G, fx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        ContT::new(WrappedTcFn::from(move |k: WrappedTcFn<B, M::Type<R>>| {
            let g = g.clone();
            ContT::run_tr(fx.clone(), WrappedTcFn::from(move |x| {
                let (g, k) = (g.clone(), k.clone());
                Computation::monadic(move || k(g.view().call(x)))
            }))
        }))
    }
}

impl<R, M, A> FunctorExt for ContT<R, M, A>
where
    R: Value,
    M: ContextConstructor,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedContTInstance<R, M>;
}

impl<R, M> Applicative for StackedContTInstance<R, M>
where
    R: Value,
    M: ContextConstructor,
{
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        ContT::new(WrappedTcFn::from(move |k: WrappedTcFn<A, M::Type<R>>| {
            let x = x.clone();
            let k = k.clone();
            Computation::monadic(move || k(x))
        }))
    }

    #[rustfmt::skip]
    fn apply<A, B, G>(fg: Self::Type<G>, fx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        ContT::new(WrappedTcFn::from(move |k: WrappedTcFn<B, M::Type<R>>| {
            let fx = fx.clone();
            ContT::run_tr(fg.clone(), WrappedTcFn::from(move |g: G| {
                let k = k.clone();
                ContT::run_tr(fx.clone(), WrappedTcFn::from(move |x: A| {
                    let (g, k) = (g.clone(), k.clone());
                    Computation::monadic(move || k(g.view().call(x)))
                }))
            }))
        }))
    }
}

impl<R, M, A> ApplicativeExt for ContT<R, M, A>
where
    R: Value,
    M: ContextConstructor,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedContTInstance<R, M>;
}

impl<R, M> Monad for StackedContTInstance<R, M>
where
    R: Value,
    M: ContextConstructor,
{
    #[rustfmt::skip]
    fn bind<A, B, G>(mx: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        ContT::new(WrappedTcFn::from(move |k: WrappedTcFn<B, M::Type<R>>| {
            let g = g.clone();
            ContT::run_tr(mx.clone(), WrappedTcFn::from(move |x| {
                let y = g.view().call(x);
                let k = k.clone();
                ContT::run_tr(y, k)
            }))
        }))
    }
}

impl<R, M, A> MonadExt for ContT<R, M, A>
where
    R: Value,
    M: ContextConstructor,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedContTInstance<R, M>;
}

#[cfg(test)]
mod tests {
    use crate::base::function::{ConcurrentFn, Curry, WrappedFn, WrappedTcFn, compose};
    use crate::control::context::applicative::{Applicative, ApplicativeExt};
    use crate::control::context::monad::{Monad, MonadExt};
    use crate::control::structure::functor::fmap;
    use crate::control::transformer::cont::{Cont, ContInstance};

    #[test]
    fn test_functor_identity_law() {
        let m = Cont::cont(WrappedTcFn::from(|k: WrappedTcFn<i32, i32>| k(10)));
        let m = fmap(&(|x| x), m);
        assert_eq!(Cont::eval(m).eval(), 10);
    }

    #[test]
    fn test_functor_composition_law() {
        let h = WrappedFn::from(|x| x * 2);
        let g = WrappedFn::from(|x| x + 3);
        let composed = g.clone().compose(h.clone());

        let m = Cont::cont(WrappedTcFn::from(|k: WrappedTcFn<i32, i32>| k(5)));
        let lhs = fmap(composed, m.clone());
        let rhs = fmap(g, fmap(h, m));
        assert_eq!(Cont::eval(lhs).eval(), 13);
        assert_eq!(Cont::eval(rhs).eval(), 13);
    }

    #[test]
    fn test_applicative_identity_law() {
        let m = Cont::cont(WrappedTcFn::from(|k: WrappedTcFn<i32, i32>| k(10)));
        let m = ContInstance::pure(&(|x| x)).apply(m);
        assert_eq!(Cont::eval(m).eval(), 10);
    }

    #[test]
    fn test_applicative_homomorphism_law() {
        let g = WrappedFn::from(|x| x * 2);
        let lhs = ContInstance::pure(g.clone()).apply(ContInstance::pure(3));
        let rhs = ContInstance::pure(g(3));
        assert_eq!(Cont::eval(lhs).eval(), Cont::eval(rhs).eval());
    }

    #[test]
    fn test_applicative_interchange_law() {
        let u = ContInstance::pure(WrappedFn::from(|x| x + 10));
        let y = 5;

        let lhs = u.clone().apply(ContInstance::pure(y));
        let rhs = ContInstance::pure(WrappedFn::from(move |g: WrappedFn<i32, i32>| g(y))).apply(u);
        assert_eq!(Cont::eval(lhs).eval(), Cont::eval(rhs).eval());
    }

    #[test]
    fn test_applicative_composition_law() {
        let u = ContInstance::pure(WrappedFn::from(|x: i32| x + 1));
        let v = ContInstance::pure(WrappedFn::from(|x: i32| x * 2));
        let w = ContInstance::pure(4);

        let lhs = ContInstance::pure(WrappedFn::curry(compose))
            .apply(u.clone())
            .apply(v.clone())
            .apply(w.clone());
        let rhs = u.apply(v.apply(w));
        assert_eq!(Cont::eval(lhs).eval(), Cont::eval(rhs).eval());
    }

    #[test]
    fn test_monad_left_identity_law() {
        let g = WrappedFn::from(|x| ContInstance::ret(x * 2));
        let lhs = ContInstance::ret(3).bind(g.clone());
        let rhs = g(3);
        assert_eq!(Cont::eval(lhs).eval(), Cont::eval(rhs).eval());
    }

    #[test]
    fn test_monad_right_identity_law() {
        let m = Cont::cont(WrappedTcFn::from(|k: WrappedTcFn<i32, i32>| k(7)));
        let m = m.bind(&ContInstance::pure);
        assert_eq!(Cont::eval(m).eval(), 7);
    }

    #[test]
    fn test_monad_associativity_law() {
        let g = WrappedFn::from(|x| ContInstance::ret(x + 1));
        let h = WrappedFn::from(|x| ContInstance::ret(x * 2));

        let m = Cont::cont(WrappedTcFn::from(|k: WrappedTcFn<i32, i32>| k(5)));
        let lhs = m.clone().bind(g.clone()).bind(h.clone());
        let rhs = m.bind(WrappedFn::from(move |x| g(x).bind(h.clone())));
        assert_eq!(Cont::eval(lhs).eval(), Cont::eval(rhs).eval());
    }
}
