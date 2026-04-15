use crate::base::function::{ConcurrentFn, WrappedFn};
use crate::base::value::Value;
use crate::control::context::alternative::Alternative;
use crate::control::context::monad::Monad;
use crate::control::transformer::MonadTrans;
use crate::control::transformer::logic::{LogicTInstance, StackedLogicTInstance};
use crate::data::list::{List, ListInstance};
use crate::data::maybe::Maybe;

pub trait MonadLogic: Monad + Alternative {
    #[allow(clippy::type_complexity)]
    fn split<A>(xs: Self::Type<A>) -> Self::Type<Maybe<(A, Self::Type<A>)>>
    where
        A: Value;

    fn interleave<A>(xs: Self::Type<A>, ys: Self::Type<A>) -> Self::Type<A>
    where
        A: Value,
    {
        Self::bind(
            Self::split(xs),
            WrappedFn::from(move |xs| match xs {
                Maybe::Nothing => ys.clone(),
                Maybe::Just((x, xs)) => Self::alt(Self::pure(x), Self::interleave(ys.clone(), xs)),
            }),
        )
    }

    fn fair_bind<A, B, G>(xs: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        Self::bind(
            Self::split(xs),
            WrappedFn::from(move |xs| match xs {
                Maybe::Nothing => Self::fallback(),
                Maybe::Just((x, xs)) => {
                    let ys = g.view().call(x);
                    let yss = Self::fair_bind(xs, g.clone());
                    Self::interleave(ys, yss)
                }
            }),
        )
    }

    fn ifte<A, B, G>(xs: Self::Type<A>, then_clause: G, else_clause: Self::Type<B>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        Self::bind(
            Self::split(xs),
            WrappedFn::from(move |cons| match cons {
                Maybe::Nothing => else_clause.clone(),
                Maybe::Just((x, xs)) => {
                    let ys = then_clause.view().call(x);
                    let yss = Self::bind(xs, then_clause.clone());
                    Self::alt(ys, yss)
                }
            }),
        )
    }

    fn once<A>(xs: Self::Type<A>) -> Self::Type<A>
    where
        A: Value,
    {
        Self::bind(
            Self::split(xs),
            &(|xs| match xs {
                Maybe::Nothing => Self::fallback(),
                Maybe::Just((x, _)) => Self::pure(x),
            }),
        )
    }

    fn lnot<A>(xs: Self::Type<A>) -> Self::Type<()>
    where
        A: Value,
    {
        Self::bind(
            Self::split(xs),
            &(|xs| match xs {
                Maybe::Nothing => Self::pure(()),
                Maybe::Just(_) => Self::fallback(),
            }),
        )
    }
}

impl<M> MonadLogic for StackedLogicTInstance<M>
where
    M: Monad + Alternative,
{
    fn split<A>(xs: Self::Type<A>) -> Self::Type<Maybe<(A, Self::Type<A>)>>
    where
        A: Value,
    {
        LogicTInstance::lift(xs.decompose())
    }
}

impl MonadLogic for ListInstance {
    fn split<A>(xs: Self::Type<A>) -> Self::Type<Maybe<(A, Self::Type<A>)>>
    where
        A: Value,
    {
        List::singleton(xs.decompose())
    }
}
