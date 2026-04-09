use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::context::monad::{Monad, MonadExt};
use crate::data::either::{Either, EitherInstance};

impl<E> Monad for EitherInstance<E>
where
    E: Value,
{
    fn bind<A, B, G>(mx: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        match mx {
            Either::Right(x) => g.view().call(x),
            Either::Left(e) => Either::Left(e),
        }
    }
}

impl<E, A> MonadExt for Either<E, A>
where
    E: Value,
    A: Value,
{
    type Wrapped = A;
    type Instance = EitherInstance<E>;
}

#[cfg(test)]
mod tests {
    use crate::base::function::WrappedFn;

    use super::*;

    #[test]
    fn test_bind_right() {
        let m = Either::<&str, i32>::Right(10);
        assert_eq!(
            m.bind(WrappedFn::from(|x| Either::Right(x + 1))),
            Either::Right(11)
        );
    }

    #[test]
    fn test_bind_left() {
        let m: Either<&str, i32> = Either::Left("err");
        assert_eq!(
            m.bind(WrappedFn::from(|x| Either::Right(x + 1))),
            Either::Left("err")
        );
    }

    #[test]
    fn test_monad_left_identity_law() {
        let g = WrappedFn::from(|x| Either::<&str, i32>::Right(x * 2));
        let lhs = EitherInstance::ret(3).bind(g.clone());
        let rhs = g(3);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_monad_right_identity_law() {
        let m = Either::<&str, i32>::Right(42);
        assert_eq!(m.clone().bind(WrappedFn::from(EitherInstance::ret)), m);

        let m: Either<&str, i32> = Either::Left("err");
        assert_eq!(m.clone().bind(WrappedFn::from(EitherInstance::ret)), m);
    }

    #[test]
    fn test_monad_associativity_law() {
        let g = WrappedFn::from(|x| Either::<&str, i32>::Right(x + 1));
        let h = WrappedFn::from(|x| Either::<&str, i32>::Right(x * 2));
        let m = Either::<&str, i32>::Right(3);
        let lhs = m.clone().bind(g.clone()).bind(h.clone());
        let rhs = m.bind(WrappedFn::from(move |x| g(x).bind(h.clone())));
        assert_eq!(lhs, Either::Right(8));
        assert_eq!(lhs, rhs);

        let g = WrappedFn::from(|x| Either::<&str, i32>::Right(x + 1));
        let h = WrappedFn::from(|x| Either::<&str, i32>::Right(x * 2));
        let m: Either<&str, i32> = Either::Left("err");
        let lhs = m.clone().bind(g.clone()).bind(h.clone());
        let rhs = m.bind(WrappedFn::from(move |x| g(x).bind(h.clone())));
        assert_eq!(lhs, Either::Left("err"));
        assert_eq!(lhs, rhs);
    }
}
