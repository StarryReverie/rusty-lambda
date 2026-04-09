use crate::base::value::{StaticConcurrent, Value};
use crate::control::context::monad::{Monad, MonadExt};

pub trait TransConstructor: Clone + StaticConcurrent {
    type Type<M, A>: StaticConcurrent
    where
        M: Monad,
        A: Value;

    type Stacked<M>: StackedMonadTrans<Transformer = Self>
    where
        M: Monad;
}

pub trait MonadTrans: TransConstructor {
    fn lift<A, MA>(mx: MA) -> Self::Type<MA::Instance, A>
    where
        A: Value,
        MA: MonadExt<Wrapped = A> + Value,
        Self::Stacked<MA::Instance>: Monad<Type<A> = Self::Type<MA::Instance, A>>;
}

pub trait StackedMonadTrans: Monad {
    type Transformer: MonadTrans;
}
