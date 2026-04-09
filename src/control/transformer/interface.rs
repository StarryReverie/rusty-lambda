use crate::base::value::{StaticConcurrent, Value};
use crate::control::context::monad::Monad;

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
    fn lift<M, A>(mx: M::Type<A>) -> Self::Type<M, A>
    where
        M: Monad,
        A: Value,
        Self::Stacked<M>: Monad<Type<A> = Self::Type<M, A>>;
}

pub trait StackedMonadTrans: Monad {
    type Transformer: MonadTrans;
}
