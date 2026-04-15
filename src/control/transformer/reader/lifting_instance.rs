use crate::base::function::{ConcurrentFn, WrappedFn};
use crate::base::value::Value;
use crate::control::transformer::MonadTrans;
use crate::control::transformer::cont::MonadCont;
use crate::control::transformer::except::MonadExcept;
use crate::control::transformer::logic::MonadLogic;
use crate::control::transformer::reader::{ReaderT, ReaderTInstance, StackedReaderTInstance};
use crate::control::transformer::state::MonadState;
use crate::control::transformer::writer::MonadWriter;
use crate::data::maybe::Maybe;

impl<R, M> MonadCont for StackedReaderTInstance<R, M>
where
    R: Value,
    M: MonadCont,
{
    fn call_cc<A, B>(
        suspended: WrappedFn<WrappedFn<A, Self::Type<B>>, Self::Type<A>>,
    ) -> Self::Type<A>
    where
        A: Value,
        B: Value,
    {
        ReaderT::new(WrappedFn::from(move |env: R| {
            let suspended = suspended.clone();
            M::call_cc(WrappedFn::from(move |escape: WrappedFn<A, M::Type<B>>| {
                ReaderT::run_tr(
                    suspended(WrappedFn::from(move |x: A| {
                        let escape = escape.clone();
                        ReaderT::new(WrappedFn::from(move |_| escape(x.clone())))
                    })),
                    env.clone(),
                )
            }))
        }))
    }
}

impl<R, M> MonadExcept for StackedReaderTInstance<R, M>
where
    R: Value,
    M: MonadExcept,
{
    type Error = M::Error;

    fn throw_error<A>(error: Self::Error) -> Self::Type<A>
    where
        A: Value,
    {
        ReaderTInstance::lift(M::throw_error(error))
    }

    fn catch_error<A, G>(fallible: Self::Type<A>, handler: G) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Error, Output = Self::Type<A>>>,
    {
        ReaderT::new(WrappedFn::from(move |env: R| {
            let handler = handler.clone();
            M::catch_error(
                ReaderT::run_tr(&fallible, env.clone()),
                WrappedFn::from(move |error| {
                    ReaderT::run_tr(handler.view().call(error), env.clone())
                }),
            )
        }))
    }
}

impl<R, M> MonadLogic for StackedReaderTInstance<R, M>
where
    R: Value,
    M: MonadLogic,
{
    fn split<A>(xs: Self::Type<A>) -> Self::Type<Maybe<(A, Self::Type<A>)>>
    where
        A: Value,
    {
        ReaderT::new(WrappedFn::from(move |env| {
            M::fmap(
                &(|cons| match cons {
                    Maybe::Nothing => Maybe::Nothing,
                    Maybe::Just((x, xs)) => Maybe::Just((x, ReaderTInstance::lift(xs))),
                }),
                M::split(ReaderT::run_tr(&xs, env)),
            )
        }))
    }
}

impl<R, M> MonadState for StackedReaderTInstance<R, M>
where
    R: Value,
    M: MonadState,
{
    type State = M::State;

    fn state<A, G>(run: G) -> Self::Type<A>
    where
        A: Value,
        G: Into<WrappedFn<Self::State, (A, Self::State)>>,
    {
        ReaderTInstance::lift(M::state(run))
    }
}

impl<R, M> MonadWriter for StackedReaderTInstance<R, M>
where
    R: Value,
    M: MonadWriter,
{
    type Log = M::Log;

    fn writer<A>(entries: (A, Self::Log)) -> Self::Type<A>
    where
        A: Value,
    {
        ReaderTInstance::lift(M::writer(entries))
    }

    fn listen<A>(context: Self::Type<A>) -> Self::Type<(A, Self::Log)>
    where
        A: Value,
    {
        ReaderT::new(WrappedFn::from(move |env: R| {
            M::listen(ReaderT::run_tr(&context, env))
        }))
    }

    fn pass<A, G>(context: Self::Type<(A, G)>) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Log, Output = Self::Log>>,
    {
        ReaderT::new(WrappedFn::from(move |env: R| {
            M::pass(ReaderT::run_tr(&context, env))
        }))
    }
}
