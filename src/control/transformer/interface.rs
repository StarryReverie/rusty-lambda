use crate::base::value::{StaticConcurrent, Value};
use crate::control::context::monad::{Monad, MonadExt};

pub trait TransConstructor {
    type Type<M, A>: StaticConcurrent
    where
        M: Monad + 'static,
        A: StaticConcurrent;

    type Stacked<M>: StackedMonadTrans<Transformer = Self> + 'static
    where
        M: Monad + 'static;
}

pub trait MonadTrans: TransConstructor {
    fn lift<A, MA>(mx: MA) -> Self::Type<MA::Instance, A>
    where
        A: Value,
        MA: MonadExt<Wrapped = A, Instance: StaticConcurrent> + Value,
        Self::Stacked<MA::Instance>: Monad<Type<A> = Self::Type<MA::Instance, A>>;
}

pub trait StackedMonadTrans: Monad {
    type Transformer: MonadTrans;
}
