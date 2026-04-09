use crate::base::function::{ConcurrentFn, WrappedFn, WrappedFnInstance};
use crate::base::value::Value;
use crate::control::context::applicative::{Applicative, ApplicativeExt};
use crate::control::context::monad::{Monad, MonadExt};
use crate::control::structure::functor::{Functor, FunctorExt};
use crate::control::transformer::state::StateT;
use crate::control::transformer::state::transformer::StackedStateTInstance;

impl<S, M> Functor for StackedStateTInstance<S, M>
where
    S: Value,
    M: Functor,
{
    fn fmap<A, B, G>(g: G, fx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        StateT(WrappedFnInstance::fmap(
            WrappedFn::from(move |mx| {
                let g = g.clone();
                M::fmap(
                    WrappedFn::from(move |(x, s): (A, S)| (g.view().call(x), s)),
                    mx,
                )
            }),
            fx.0,
        ))
    }
}

impl<S, M, A> FunctorExt for StateT<S, M, A>
where
    S: Value,
    M: Functor,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedStateTInstance<S, M>;
}

impl<S, M> Applicative for StackedStateTInstance<S, M>
where
    S: Value,
    M: Monad,
{
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        StateT(WrappedFn::from(move |s| M::pure((x.clone(), s))))
    }

    fn apply<A, B, G>(smg: Self::Type<G>, smx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        StateT(WrappedFn::from(move |s: S| {
            let mg = StateT::run_tr(smg.clone(), s);
            let smx = smx.clone();
            M::bind(
                mg,
                WrappedFn::from(move |(g, s): (G, S)| {
                    let mx = StateT::run_tr(smx.clone(), s);
                    M::fmap(
                        WrappedFn::from(move |(x, s): (A, S)| (g.view().call(x), s)),
                        mx,
                    )
                }),
            )
        }))
    }
}

impl<S, M, A> ApplicativeExt for StateT<S, M, A>
where
    S: Value,
    M: Monad,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedStateTInstance<S, M>;
}

impl<S, M> Monad for StackedStateTInstance<S, M>
where
    S: Value,
    M: Monad,
{
    fn bind<A, B, G>(smx: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        StateT(WrappedFn::from(move |s: S| {
            let mx = StateT::run_tr(smx.clone(), s);
            let g = g.clone();
            M::bind(
                mx,
                WrappedFn::from(move |(x, s): (A, S)| {
                    let smy = g.view().call(x);
                    StateT::run_tr(smy, s)
                }),
            )
        }))
    }
}

impl<S, M, A> MonadExt for StateT<S, M, A>
where
    S: Value,
    M: Monad,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedStateTInstance<S, M>;
}
