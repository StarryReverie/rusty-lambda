use crate::base::computation::Thunk;
use crate::base::function::{ConcurrentFn, WrappedFn};
use crate::base::value::Value;
use crate::control::context::applicative::{Applicative, ApplicativeExt};
use crate::control::context::monad::{Monad, MonadExt};
use crate::control::structure::functor::{Functor, FunctorExt};
use crate::control::transformer::logic::{LogicT, StackedLogicTInstance};
use crate::data::maybe::Maybe;

impl<M> Functor for StackedLogicTInstance<M>
where
    M: Functor,
{
    fn fmap<A, B, G>(g: G, fx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        LogicT::new(Thunk::lazy(move || {
            M::fmap(
                WrappedFn::from(move |xs| match xs {
                    Maybe::Nothing => Maybe::Nothing,
                    Maybe::Just((x, xs)) => {
                        let x = g.view().call(x);
                        let xs = Self::fmap(g.clone(), xs);
                        Maybe::Just((x, xs))
                    }
                }),
                fx.decompose(),
            )
        }))
    }
}

impl<M, A> FunctorExt for LogicT<M, A>
where
    M: Functor,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedLogicTInstance<M>;
}

impl<M> Applicative for StackedLogicTInstance<M>
where
    M: Monad,
{
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        LogicT::singleton(x)
    }

    fn apply<A, B, G>(gs: Self::Type<G>, xs: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        Self::bind(gs, WrappedFn::from(move |g| Self::fmap(g, xs.clone())))
    }
}

impl<M, A> ApplicativeExt for LogicT<M, A>
where
    M: Monad,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedLogicTInstance<M>;
}

impl<M> Monad for StackedLogicTInstance<M>
where
    M: Monad,
{
    fn bind<A, B, G>(xs: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        LogicT::new(Thunk::lazy(move || {
            M::bind(
                xs.decompose(),
                WrappedFn::from(move |xs| match xs {
                    Maybe::Nothing => M::pure(Maybe::Nothing),
                    Maybe::Just((x, xs)) => {
                        let ys = g.view().call(x);
                        let yss = Self::bind(xs, g.clone());
                        ys.append(yss).decompose()
                    }
                }),
            )
        }))
    }
}

impl<M, A> MonadExt for LogicT<M, A>
where
    M: Monad,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedLogicTInstance<M>;
}

#[cfg(test)]
mod tests {
    use crate::base::function::{ConcurrentFn, Curry, WrappedFn, compose};
    use crate::control::context::applicative::{Applicative, ApplicativeExt};
    use crate::control::context::monad::{Monad, MonadExt};
    use crate::control::structure::functor::fmap;
    use crate::control::structure::functor::identity::{Identity, IdentityInstance};
    use crate::data::maybe::Maybe;

    use super::*;

    type Logic<A> = LogicT<IdentityInstance, A>;
    type LogicInstance = StackedLogicTInstance<IdentityInstance>;

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
    fn test_functor_identity_law() {
        let xs = LogicT::cons(1, LogicT::cons(2, LogicT::cons(3, LogicT::empty())));
        let xs = fmap(&(|x| x), xs);
        assert_eq!(run_logic(&xs), vec![1, 2, 3]);
    }

    #[test]
    fn test_functor_composition_law() {
        let h = WrappedFn::from(|x| x * 2);
        let g = WrappedFn::from(|x| x + 3);
        let composed = g.clone().compose(h.clone());

        let xs = LogicT::cons(1, LogicT::cons(2, LogicT::cons(3, LogicT::empty())));
        let lhs = fmap(composed, xs.clone());
        let rhs = fmap(g, fmap(h, xs));
        assert_eq!(run_logic(&lhs), vec![5, 7, 9]);
        assert_eq!(run_logic(&rhs), vec![5, 7, 9]);
    }

    #[test]
    fn test_applicative_identity_law() {
        let xs = LogicT::cons(1, LogicT::cons(2, LogicT::empty()));
        let xs = LogicInstance::pure(&(|x| x)).apply(xs);
        assert_eq!(run_logic(&xs), vec![1, 2]);
    }

    #[test]
    fn test_applicative_homomorphism_law() {
        let g = WrappedFn::from(|x| x * 2);
        let lhs = LogicInstance::pure(g.clone()).apply(LogicInstance::pure(3));
        let rhs = LogicInstance::pure(g(3));
        assert_eq!(run_logic(&lhs), run_logic(&rhs));
        assert_eq!(run_logic(&lhs), vec![6]);
    }

    #[test]
    fn test_applicative_interchange_law() {
        let u = LogicT::cons(
            WrappedFn::from(|x| x + 10),
            LogicT::cons(WrappedFn::from(|x| x * 2), LogicT::empty()),
        );
        let y = 5;

        let lhs = u.clone().apply(LogicInstance::pure(y));
        let rhs = LogicInstance::pure(WrappedFn::from(move |g: WrappedFn<i32, i32>| g(y))).apply(u);
        assert_eq!(run_logic(&lhs), run_logic(&rhs));
        assert_eq!(run_logic(&lhs), vec![15, 10]);
    }

    #[test]
    fn test_applicative_composition_law() {
        let u = LogicT::cons(
            WrappedFn::from(|x| x + 1),
            LogicT::cons(WrappedFn::from(|x| x * 10), LogicT::empty()),
        );
        let v = LogicT::cons(
            WrappedFn::from(|x| x * 2),
            LogicT::cons(WrappedFn::from(|x| x + 3), LogicT::empty()),
        );
        let w = LogicT::cons(4, LogicT::cons(5, LogicT::empty()));

        let lhs = LogicInstance::pure(WrappedFn::curry(compose))
            .apply(u.clone())
            .apply(v.clone())
            .apply(w.clone());
        let rhs = u.apply(v.apply(w));
        assert_eq!(run_logic(&lhs), run_logic(&rhs));
    }

    #[test]
    fn test_monad_left_identity_law() {
        let g = WrappedFn::from(|x| LogicT::cons(x * 10, LogicT::cons(x * 100, LogicT::empty())));
        let lhs = LogicInstance::ret(3).bind(g.clone());
        let rhs = g(3);
        assert_eq!(run_logic(&lhs), run_logic(&rhs));
        assert_eq!(run_logic(&lhs), vec![30, 300]);
    }

    #[test]
    fn test_monad_right_identity_law() {
        let xs = LogicT::cons(1, LogicT::cons(2, LogicT::cons(3, LogicT::empty())));
        let xs = xs.bind(&LogicInstance::ret);
        assert_eq!(run_logic(&xs), vec![1, 2, 3]);
    }

    #[test]
    fn test_monad_associativity_law() {
        let g =
            WrappedFn::from(|x: i32| LogicT::cons(x + 1, LogicT::cons(x * 10, LogicT::empty())));
        let h = WrappedFn::from(|x: i32| LogicT::cons(x * 2, LogicT::empty()));

        let xs = LogicT::cons(1, LogicT::cons(2, LogicT::empty()));
        let lhs = xs.clone().bind(g.clone()).bind(h.clone());
        let rhs = xs.bind(WrappedFn::from(move |x| g(x).bind(h.clone())));
        assert_eq!(run_logic(&lhs), run_logic(&rhs));
    }
}
