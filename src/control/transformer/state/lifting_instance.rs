use crate::base::function::{ConcurrentFn, WrappedFn, constv};
use crate::base::value::Value;
use crate::control::transformer::MonadTrans;
use crate::control::transformer::cont::MonadCont;
use crate::control::transformer::except::MonadExcept;
use crate::control::transformer::logic::MonadLogic;
use crate::control::transformer::reader::MonadReader;
use crate::control::transformer::state::{StackedStateTInstance, StateT, StateTInstance};
use crate::control::transformer::writer::MonadWriter;
use crate::data::maybe::Maybe;

impl<S, M> MonadCont for StackedStateTInstance<S, M>
where
    S: Value,
    M: MonadCont,
{
    fn call_cc<A, B>(
        suspended: WrappedFn<WrappedFn<A, Self::Type<B>>, Self::Type<A>>,
    ) -> Self::Type<A>
    where
        A: Value,
        B: Value,
    {
        StateT::new(WrappedFn::from(move |state: S| {
            let suspended = suspended.clone();
            M::call_cc(WrappedFn::from(
                move |escape: WrappedFn<(A, S), M::Type<(B, S)>>| {
                    let (state, state2) = (state.clone(), state.clone());
                    StateT::run_tr(
                        suspended(WrappedFn::from(move |x: A| {
                            let (escape, state) = (escape.clone(), state.clone());
                            StateT::new(WrappedFn::from(move |_| {
                                escape((x.clone(), state.clone()))
                            }))
                        })),
                        state2,
                    )
                },
            ))
        }))
    }
}

impl<S, M> MonadExcept for StackedStateTInstance<S, M>
where
    S: Value,
    M: MonadExcept,
{
    type Error = M::Error;

    fn throw_error<A>(error: Self::Error) -> Self::Type<A>
    where
        A: Value,
    {
        StateTInstance::lift(M::throw_error(error))
    }

    fn catch_error<A, G>(fallible: Self::Type<A>, handler: G) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Error, Output = Self::Type<A>>>,
    {
        StateT::new(WrappedFn::from(move |state: S| {
            let handler = handler.clone();
            M::catch_error(
                StateT::run_tr(&fallible, state.clone()),
                WrappedFn::from(move |error| {
                    StateT::run_tr(handler.view().call(error), state.clone())
                }),
            )
        }))
    }
}

impl<S, M> MonadLogic for StackedStateTInstance<S, M>
where
    S: Value,
    M: MonadLogic,
{
    fn split<A>(xs: Self::Type<A>) -> Self::Type<Maybe<(A, Self::Type<A>)>>
    where
        A: Value,
    {
        StateT::new(WrappedFn::from(move |state: S| {
            let cons = M::split(StateT::run_tr(&xs, state.clone()));
            M::fmap(
                WrappedFn::from(move |cons| match cons {
                    Maybe::Nothing => (Maybe::Nothing, state.clone()),
                    Maybe::Just(((x, state), xs)) => {
                        let xs = StateT::new(constv(xs));
                        (Maybe::Just((x, xs)), state)
                    }
                }),
                cons,
            )
        }))
    }

    fn interleave<A>(xs: Self::Type<A>, ys: Self::Type<A>) -> Self::Type<A>
    where
        A: Value,
    {
        StateT::new(WrappedFn::from(move |state: S| {
            let xs = StateT::run_tr(&xs, state.clone());
            let ys = StateT::run_tr(&ys, state);
            M::interleave(xs, ys)
        }))
    }

    fn fair_bind<A, B, G>(xs: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        StateT::new(WrappedFn::from(move |state: S| {
            let xs = StateT::run_tr(&xs, state.clone());
            let g = g.clone();
            M::fair_bind(
                xs,
                WrappedFn::from(move |(x, state)| {
                    let ys = g.view().call(x);
                    StateT::run_tr(ys, state)
                }),
            )
        }))
    }

    fn ifte<A, B, G>(xs: Self::Type<A>, then_clause: G, else_clause: Self::Type<B>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        StateT::new(WrappedFn::from(move |state: S| {
            let xs = StateT::run_tr(&xs, state.clone());
            let then_clause = then_clause.clone();
            M::ifte(
                xs,
                WrappedFn::from(move |(x, state)| {
                    let ys = then_clause.view().call(x);
                    StateT::run_tr(ys, state)
                }),
                StateT::run_tr(&else_clause, state),
            )
        }))
    }
}

impl<S, M> MonadReader for StackedStateTInstance<S, M>
where
    S: Value,
    M: MonadReader,
{
    type Environment = M::Environment;

    fn ask() -> Self::Type<Self::Environment> {
        StateTInstance::lift(M::ask())
    }

    fn local<A, G>(localize: G, context: Self::Type<A>) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Environment, Output = Self::Environment>>,
    {
        StateT::new(WrappedFn::from(move |state| {
            M::local(localize.clone(), StateT::run_tr(&context, state))
        }))
    }
}

impl<S, M> MonadWriter for StackedStateTInstance<S, M>
where
    S: Value,
    M: MonadWriter,
{
    type Log = M::Log;

    fn writer<A>(entries: (A, Self::Log)) -> Self::Type<A>
    where
        A: Value,
    {
        StateTInstance::lift(M::writer(entries))
    }

    fn listen<A>(context: Self::Type<A>) -> Self::Type<(A, Self::Log)>
    where
        A: Value,
    {
        StateT::new(WrappedFn::from(move |state| {
            M::fmap(
                &(|((x, state), log)| ((x, log), state)),
                M::listen(StateT::run_tr(&context, state)),
            )
        }))
    }

    fn pass<A, G>(context: Self::Type<(A, G)>) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Log, Output = Self::Log>>,
    {
        StateT::new(WrappedFn::from(move |state| {
            M::pass(M::fmap(
                &(|((x, map), state)| ((x, state), map)),
                StateT::run_tr(&context, state),
            ))
        }))
    }
}
