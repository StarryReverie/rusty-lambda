use crate::base::function::{ConcurrentFn, WrappedFn};
use crate::base::value::Value;
use crate::control::structure::monoid::Monoid;
use crate::control::transformer::MonadTrans;
use crate::control::transformer::cont::MonadCont;
use crate::control::transformer::except::MonadExcept;
use crate::control::transformer::logic::MonadLogic;
use crate::control::transformer::reader::MonadReader;
use crate::control::transformer::state::MonadState;
use crate::control::transformer::writer::{StackedWriterTInstance, WriterT, WriterTInstance};
use crate::data::maybe::Maybe;

impl<W, M> MonadCont for StackedWriterTInstance<W, M>
where
    W: Monoid + Value,
    M: MonadCont,
{
    fn call_cc<A, B>(
        suspended: WrappedFn<WrappedFn<A, Self::Type<B>>, Self::Type<A>>,
    ) -> Self::Type<A>
    where
        A: Value,
        B: Value,
    {
        WriterT::new(M::call_cc(WrappedFn::from(
            move |escape: WrappedFn<(A, W), M::Type<(B, W)>>| {
                WriterT::run_tr(suspended(WrappedFn::from(move |x| {
                    WriterT::new(escape((x, W::empty())))
                })))
            },
        )))
    }
}

impl<W, M> MonadExcept for StackedWriterTInstance<W, M>
where
    W: Monoid + Value,
    M: MonadExcept,
{
    type Error = M::Error;

    fn throw_error<A>(error: Self::Error) -> Self::Type<A>
    where
        A: Value,
    {
        WriterTInstance::lift(M::throw_error(error))
    }

    fn catch_error<A, G>(fallible: Self::Type<A>, handler: G) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Error, Output = Self::Type<A>>>,
    {
        WriterT::new(M::catch_error(
            WriterT::run_tr(fallible),
            WrappedFn::from(move |error| WriterT::run_tr(handler.view().call(error))),
        ))
    }
}

impl<W, M> MonadLogic for StackedWriterTInstance<W, M>
where
    W: Monoid + Value,
    M: MonadLogic,
{
    fn split<A>(xs: Self::Type<A>) -> Self::Type<Maybe<(A, Self::Type<A>)>>
    where
        A: Value,
    {
        WriterT::new(M::fmap(
            &(|cons| match cons {
                Maybe::Nothing => (Maybe::Nothing, W::empty()),
                Maybe::Just(((x, log), xs)) => (Maybe::Just((x, WriterT::new(xs))), log),
            }),
            M::split(WriterT::run_tr(xs)),
        ))
    }
}

impl<W, M> MonadReader for StackedWriterTInstance<W, M>
where
    W: Monoid + Value,
    M: MonadReader,
{
    type Environment = M::Environment;

    fn ask() -> Self::Type<Self::Environment> {
        WriterTInstance::lift(M::ask())
    }

    fn local<A, G>(localize: G, context: Self::Type<A>) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Environment, Output = Self::Environment>>,
    {
        WriterT::new(M::local(localize, WriterT::run_tr(context)))
    }
}

impl<W, M> MonadState for StackedWriterTInstance<W, M>
where
    W: Monoid + Value,
    M: MonadState,
{
    type State = M::State;

    fn state<A, G>(run: G) -> Self::Type<A>
    where
        A: Value,
        G: Into<WrappedFn<Self::State, (A, Self::State)>>,
    {
        WriterTInstance::lift(M::state(run))
    }
}
