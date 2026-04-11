use crate::base::function::{ConcurrentFn, WrappedFn, WrappedFnInstance};
use crate::base::value::Value;
use crate::control::context::applicative::{Applicative, ApplicativeExt};
use crate::control::context::monad::{Monad, MonadExt};
use crate::control::structure::functor::{Functor, FunctorExt};
use crate::control::transformer::reader::{ReaderT, StackedReaderTInstance};

impl<R, M> Functor for StackedReaderTInstance<R, M>
where
    R: Value,
    M: Functor,
{
    fn fmap<A, B, G>(g: G, fx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        ReaderT::new(WrappedFn::from(move |env| {
            let x = ReaderT::run_tr(&fx, env);
            let g = g.clone();
            M::fmap(WrappedFn::from(move |x| g.view().call(x)), x)
        }))
    }
}

impl<R, M, A> FunctorExt for ReaderT<R, M, A>
where
    R: Value,
    M: Functor,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedReaderTInstance<R, M>;
}

impl<R, M> Applicative for StackedReaderTInstance<R, M>
where
    R: Value,
    M: Applicative,
{
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        ReaderT::new(WrappedFnInstance::pure(M::pure(x)))
    }

    fn apply<A, B, G>(fg: Self::Type<G>, fx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        ReaderT::new(WrappedFn::from(move |env: R| {
            let g = ReaderT::run_tr(&fg, env.clone());
            let x = ReaderT::run_tr(&fx, env);
            M::apply(g, x)
        }))
    }
}

impl<R, M, A> ApplicativeExt for ReaderT<R, M, A>
where
    R: Value,
    M: Applicative,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedReaderTInstance<R, M>;
}

impl<R, M> Monad for StackedReaderTInstance<R, M>
where
    R: Value,
    M: Monad,
{
    fn bind<A, B, G>(mx: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        ReaderT::new(WrappedFn::from(move |env: R| {
            let x = ReaderT::run_tr(&mx, env.clone());
            let g = g.clone();
            M::bind(
                x,
                WrappedFn::from(move |x| {
                    let my = g.view().call(x);
                    ReaderT::run_tr(my, env.clone())
                }),
            )
        }))
    }
}

impl<R, M, A> MonadExt for ReaderT<R, M, A>
where
    R: Value,
    M: Monad,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedReaderTInstance<R, M>;
}

#[cfg(test)]
mod tests {
    use crate::base::function::{ConcurrentFn, Curry, WrappedFn, compose};
    use crate::control::context::applicative::{Applicative, ApplicativeExt};
    use crate::control::context::monad::{Monad, MonadExt};
    use crate::control::structure::functor::fmap;
    use crate::control::transformer::reader::{Reader, ReaderInstance};

    #[test]
    fn test_functor_identity_law() {
        let reader = Reader::from(|r| r + 10);
        let reader = fmap(WrappedFn::from(|x| x), reader);
        assert_eq!(Reader::run(reader, 5), 15);
    }

    #[test]
    fn test_functor_composition_law() {
        let h = WrappedFn::from(|x| x * 2);
        let g = WrappedFn::from(|x| x + 3);
        let composed = g.clone().compose(h.clone());

        let reader = Reader::from(|r| r);
        let lhs = fmap(composed, reader.clone());
        let rhs = fmap(g, fmap(h, reader));
        assert_eq!(Reader::run(lhs, 4), 11);
        assert_eq!(Reader::run(rhs, 4), 11);
    }

    #[test]
    fn test_applicative_identity_law() {
        let reader = ReaderInstance::pure(WrappedFn::from(|x| x)).apply(Reader::from(|r| r + 10));
        assert_eq!(Reader::run(reader, 5), 15);
    }

    #[test]
    fn test_applicative_homomorphism_law() {
        let h = WrappedFn::from(|x| x * 2);
        let lhs = ReaderInstance::pure(h.clone()).apply(ReaderInstance::pure(3));
        let rhs = ReaderInstance::pure(h(3));
        assert_eq!(Reader::run(lhs, 99), Reader::run(rhs, 99));
    }

    #[test]
    fn test_applicative_interchange_law() {
        let h = Reader::from(|r| WrappedFn::from(move |x| x + r));
        let x = 5;

        let lhs = h.clone().apply(ReaderInstance::pure(x));
        let rhs =
            ReaderInstance::pure(WrappedFn::from(move |g: WrappedFn<i32, i32>| g(x))).apply(h);
        assert_eq!(Reader::run(&lhs, 3), Reader::run(&rhs, 3));
        assert_eq!(Reader::run(&lhs, 10), Reader::run(&rhs, 10));
    }

    #[test]
    fn test_applicative_composition_law() {
        let g = Reader::from(|r| WrappedFn::from(move |x| x * r));
        let h = Reader::from(|r| WrappedFn::from(move |x| x + r));
        let composed = ReaderInstance::pure(WrappedFn::curry(compose))
            .apply(g.clone())
            .apply(h.clone());

        let x = ReaderInstance::pure(4);
        let lhs = composed.apply(x.clone());
        let rhs = g.apply(h.apply(x));
        assert_eq!(Reader::run(&lhs, 3), Reader::run(&rhs, 3));
        assert_eq!(Reader::run(&lhs, 10), Reader::run(&rhs, 10));
    }

    #[test]
    fn test_monad_left_identity_law() {
        let g = WrappedFn::from(|x| Reader::from(move |r| x * r));
        let lhs = ReaderInstance::ret(3).bind(g.clone());
        let rhs = g(3);
        assert_eq!(Reader::run(lhs, 10), Reader::run(rhs, 10));
    }

    #[test]
    fn test_monad_right_identity_law() {
        let m = Reader::from(|r| r + 5).bind(WrappedFn::from(|x| ReaderInstance::ret(x)));
        assert_eq!(Reader::run(m, 3), 8);
    }

    #[test]
    fn test_monad_associativity_law() {
        let g = WrappedFn::from(|x| Reader::from(move |r| x + r));
        let h = WrappedFn::from(|x| Reader::from(move |r| x * r));

        let m = Reader::from(|r| r);
        let lhs = m.clone().bind(g.clone()).bind(h.clone());
        let rhs = m.bind(WrappedFn::from(move |x| g(x).bind(h.clone())));
        assert_eq!(Reader::run(&lhs, 5), Reader::run(&rhs, 5));
        assert_eq!(Reader::run(&lhs, 5), (5 + 5) * 5);
    }
}
