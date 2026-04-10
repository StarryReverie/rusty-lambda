use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::context::monad::Monad;
use crate::control::structure::monoid::Monoid;
use crate::control::transformer::writer::{StackedWriterTInstance, WriterT};

pub trait MonadWriter: Monad {
    type Log: Monoid + Value;

    fn writer<A>(entries: (A, Self::Log)) -> Self::Type<A>
    where
        A: Value;

    fn tell(log: Self::Log) -> Self::Type<()> {
        Self::writer(((), log))
    }

    fn listen<A>(context: Self::Type<A>) -> Self::Type<(A, Self::Log)>
    where
        A: Value;

    fn pass<A, G>(context: Self::Type<(A, G)>) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Log, Output = Self::Log>>;
}

impl<W, M> MonadWriter for StackedWriterTInstance<W, M>
where
    W: Monoid + Value,
    M: Monad,
{
    type Log = W;

    fn writer<A>(entries: (A, Self::Log)) -> Self::Type<A>
    where
        A: Value,
    {
        WriterT::writer(entries)
    }

    fn listen<A>(context: Self::Type<A>) -> Self::Type<(A, Self::Log)>
    where
        A: Value,
    {
        WriterT::new(M::fmap(
            &(|(x, log): (A, Self::Log)| ((x, log.clone()), log)),
            WriterT::run_tr(context),
        ))
    }

    fn pass<A, G>(context: Self::Type<(A, G)>) -> Self::Type<A>
    where
        A: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Log, Output = Self::Log>>,
    {
        WriterT::new(M::fmap(
            &(|((x, map), log): ((A, G), Self::Log)| (x, map.view().call(log))),
            WriterT::run_tr(context),
        ))
    }
}
