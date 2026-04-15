use crate::base::computation::Computation;
use crate::base::function::{WrappedFn, WrappedTcFn};
use crate::base::value::Value;
use crate::control::context::ContextConstructor;
use crate::control::context::monad::Monad;
use crate::control::transformer::cont::{ContT, StackedContTInstance};

pub trait MonadCont: Monad {
    fn call_cc<A, B>(
        suspended: WrappedFn<WrappedFn<A, Self::Type<B>>, Self::Type<A>>,
    ) -> Self::Type<A>
    where
        A: Value,
        B: Value;
}

impl<R, M> MonadCont for StackedContTInstance<R, M>
where
    R: Value,
    M: ContextConstructor,
{
    fn call_cc<A, B>(
        suspended: WrappedFn<WrappedFn<A, Self::Type<B>>, Self::Type<A>>,
    ) -> Self::Type<A>
    where
        A: Value,
        B: Value,
    {
        ContT::new(WrappedTcFn::from(move |cur: WrappedTcFn<A, M::Type<R>>| {
            let suspended = suspended.clone();
            let cur2 = cur.clone();
            let escape = WrappedFn::from(move |x: A| {
                let (cur, x) = (cur.clone(), x.clone());
                ContT::new(WrappedTcFn::from(move |_| {
                    let (cur, x) = (cur.clone(), x.clone());
                    Computation::monadic(move || cur(x))
                }))
            });
            ContT::run_tr(suspended(escape), cur2)
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::WrappedFn;
    use crate::control::context::applicative::Applicative;
    use crate::control::context::monad::{MonadExt, when};
    use crate::control::transformer::cont::{Cont, ContInstance, MonadCont};

    #[test]
    fn test_call_cc() {
        let m = |x: i32| {
            ContInstance::call_cc(WrappedFn::from(move |exit: WrappedFn<_, _>| {
                let exit = exit.clone();
                let exit2 = exit.clone();
                ContInstance::pure(x)
                    .bind(WrappedFn::from(move |_| when(x < 0, exit(-x))))
                    .bind(WrappedFn::from(move |_| when(x > 100, exit2(-100))))
                    .bind(WrappedFn::from(move |_| ContInstance::pure(x + 1)))
            }))
        };
        assert_eq!(Cont::eval(m(-100)).eval(), 100);
        assert_eq!(Cont::eval(m(11)).eval(), 12);
        assert_eq!(Cont::eval(m(101)).eval(), -100);
    }
}
