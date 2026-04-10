use std::borrow::Borrow;

use crate::base::value::Value;
use crate::control::structure::functor::identity::{Identity, IdentityInstance};
use crate::control::transformer::reader::{ReaderT, StackedReaderTInstance};

pub type Reader<R, A> = ReaderT<R, IdentityInstance, A>;
pub type ReaderInstance<R> = StackedReaderTInstance<R, IdentityInstance>;

impl<R, A> Reader<R, A>
where
    R: Value,
    A: Value,
{
    pub fn run(reader: impl Borrow<Self>, env: R) -> A {
        Identity::run(Self::run_tr(reader, env))
    }
}
