use crate::base::computation::Computation;
use crate::base::function::WrappedTcFn;
use crate::base::value::Value;
use crate::control::structure::functor::identity::{Identity, IdentityInstance};
use crate::control::transformer::cont::{ContT, StackedContTInstance};

pub type Cont<R, A> = ContT<R, IdentityInstance, A>;
pub type ContInstance<R> = StackedContTInstance<R, IdentityInstance>;

impl<R, A> Cont<R, A>
where
    R: Value,
    A: Value,
{
    pub fn cont(cont: WrappedTcFn<WrappedTcFn<A, R>, R>) -> Self {
        Self::new(WrappedTcFn::from(move |k: WrappedTcFn<A, Identity<R>>| {
            cont(WrappedTcFn::from(move |x| k(x).map(Identity::run))).map(Identity)
        }))
    }

    pub fn run(suspended: Self, continuation: WrappedTcFn<A, Identity<R>>) -> Computation<R> {
        Self::run_tr(suspended, continuation).map(Identity::run)
    }
}

impl<R> Cont<R, R>
where
    R: Value,
{
    pub fn eval(suspended: Self) -> Computation<R> {
        Self::eval_tr(suspended).map(Identity::run)
    }
}
