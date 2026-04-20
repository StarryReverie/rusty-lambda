use crate::base::function::{ConcurrentFn, WrappedFn};
use crate::base::value::{StaticConcurrent, Value};
use crate::control::context::applicative::{Applicative, ApplicativeExt};
use crate::control::context::monad::{Monad, MonadExt};
use crate::control::structure::functor::{Functor, FunctorExt};
use crate::system::io::description::{IO, IOBindAction, IOInstance, IOMapAction, IOPureAction};

impl<C> Functor for IOInstance<C>
where
    C: StaticConcurrent,
{
    fn fmap<A, B, G>(g: G, fx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        IOMapAction::boxed(fx, g)
    }
}

impl<C, A> FunctorExt for IO<C, A>
where
    C: StaticConcurrent,
    A: Value,
{
    type Wrapped = A;
    type Instance = IOInstance<C>;
}

impl<C> Applicative for IOInstance<C>
where
    C: StaticConcurrent,
{
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        IOPureAction::boxed(x)
    }

    fn apply<A, B, G>(fg: Self::Type<G>, fx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        Self::bind(fg, WrappedFn::from(move |g| Self::fmap(g, fx.clone())))
    }
}

impl<C, A> ApplicativeExt for IO<C, A>
where
    C: StaticConcurrent,
    A: Value,
{
    type Wrapped = A;
    type Instance = IOInstance<C>;
}

impl<C> Monad for IOInstance<C>
where
    C: StaticConcurrent,
{
    fn bind<A, B, G>(mx: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        IOBindAction::boxed(mx, g)
    }
}

impl<C, A> MonadExt for IO<C, A>
where
    C: StaticConcurrent,
    A: Value,
{
    type Wrapped = A;
    type Instance = IOInstance<C>;
}

#[cfg(test)]
mod tests {
    use crate::base::function::{ConcurrentFn, Curry, WrappedFn, compose};
    use crate::base::value::Value;
    use crate::control::context::applicative::{Applicative, ApplicativeExt};
    use crate::control::context::monad::{Monad, MonadExt};
    use crate::control::structure::functor::fmap;
    use crate::system::io::description::{IO, IOInstance};

    #[derive(Default)]
    struct TestContext;

    type AppInstance = IOInstance<TestContext>;

    fn run<A>(io: IO<TestContext, A>) -> A
    where
        A: Value,
    {
        IO::run(io, &mut TestContext)
    }

    #[test]
    fn functor_identity_law() {
        let m = AppInstance::pure(10);
        let result = fmap(&|x| x, m);
        assert_eq!(run(result), 10);
    }

    #[test]
    fn functor_composition_law() {
        let h = WrappedFn::from(|x| x * 2);
        let g = WrappedFn::from(|x| x + 3);
        let composed = g.clone().compose(h.clone());

        let m = AppInstance::pure(5);
        let lhs = fmap(composed, m.clone());
        let rhs = fmap(g, fmap(h, m));
        assert_eq!(run(lhs), 13);
        assert_eq!(run(rhs), 13);
    }

    #[test]
    fn applicative_identity_law() {
        let m = AppInstance::pure(10);
        let result = AppInstance::pure(&|x| x).apply(m);
        assert_eq!(run(result), 10);
    }

    #[test]
    fn applicative_homomorphism_law() {
        let g = WrappedFn::from(|x| x * 2);
        let lhs = AppInstance::pure(g.clone()).apply(AppInstance::pure(3));
        let rhs = AppInstance::pure(g(3));
        assert_eq!(run(lhs), run(rhs));
    }

    #[test]
    fn applicative_interchange_law() {
        let u = AppInstance::pure(WrappedFn::from(|x| x + 10));
        let y = 5;

        let lhs = u.clone().apply(AppInstance::pure(y));
        let rhs = AppInstance::pure(WrappedFn::from(move |g: WrappedFn<i32, i32>| g(y))).apply(u);
        assert_eq!(run(lhs), run(rhs));
    }

    #[test]
    fn applicative_composition_law() {
        let u = AppInstance::pure(WrappedFn::from(|x| x + 1));
        let v = AppInstance::pure(WrappedFn::from(|x| x * 2));
        let w = AppInstance::pure(4);

        let lhs = AppInstance::pure(WrappedFn::curry(compose))
            .apply(u.clone())
            .apply(v.clone())
            .apply(w.clone());
        let rhs = u.apply(v.apply(w));
        assert_eq!(run(lhs), run(rhs));
    }

    #[test]
    fn monad_left_identity_law() {
        let g = WrappedFn::from(|x| AppInstance::ret(x * 2));
        let lhs = AppInstance::ret(3).bind(g.clone());
        let rhs = g(3);
        assert_eq!(run(lhs), run(rhs));
    }

    #[test]
    fn monad_right_identity_law() {
        let m = AppInstance::pure(7);
        let result = m.bind(&AppInstance::pure);
        assert_eq!(run(result), 7);
    }

    #[test]
    fn monad_associativity_law() {
        let g = WrappedFn::from(|x| AppInstance::ret(x + 1));
        let h = WrappedFn::from(|x| AppInstance::ret(x * 2));

        let m = AppInstance::pure(5);
        let lhs = m.clone().bind(g.clone()).bind(h.clone());
        let rhs = m.bind(WrappedFn::from(move |x| g(x).bind(h.clone())));
        assert_eq!(run(lhs), run(rhs));
    }
}
