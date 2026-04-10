use crate::base::value::Value;
use crate::control::structure::functor::identity::{Identity, IdentityInstance};
use crate::control::transformer::writer::{StackedWriterTInstance, WriterT};

pub type Writer<W, A> = WriterT<W, IdentityInstance, A>;
pub type WriterInstance<W> = StackedWriterTInstance<W, IdentityInstance>;

impl<W, A> Writer<W, A>
where
    W: Value,
    A: Value,
{
    pub fn run(trans: Self) -> (A, W) {
        Identity::run(WriterT::run_tr(trans))
    }

    pub fn eval(trans: Self) -> A {
        Self::run(trans).0
    }

    pub fn exec(trans: Self) -> W {
        Self::run(trans).1
    }
}
