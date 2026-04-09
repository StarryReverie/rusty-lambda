use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::context::applicative::{Applicative, ApplicativeExt};
use crate::data::either::{Either, EitherInstance};

impl<E> Applicative for EitherInstance<E>
where
    E: Value,
{
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        Either::Right(x)
    }

    fn apply<A, B, G>(fg: Self::Type<G>, fx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        match (fg, fx) {
            (Either::Right(g), Either::Right(x)) => Either::Right(g.view().call(x)),
            (Either::Left(e), _) | (_, Either::Left(e)) => Either::Left(e),
        }
    }
}

impl<E, A> ApplicativeExt for Either<E, A>
where
    E: Value,
    A: Value,
{
    type Wrapped = A;
    type Instance = EitherInstance<E>;
}

#[cfg(test)]
mod tests {
    use crate::base::function::{Curry, WrappedFn, compose};

    use super::*;

    #[test]
    fn test_pure() {
        assert_eq!(EitherInstance::pure(42), Either::<&str, i32>::Right(42));
    }

    #[test]
    fn test_apply() {
        let fg = Either::<&str, _>::Right(WrappedFn::from(|x| x + 1));
        assert_eq!(fg.apply(Either::Right(1)), Either::Right(2));

        let fg: Either<&str, WrappedFn<i32, i32>> = Either::Left("err");
        assert_eq!(fg.apply(Either::Right(1)), Either::Left("err"));

        let fg = Either::Right(WrappedFn::from(|x: i32| x + 1));
        assert_eq!(fg.apply(Either::Left("err")), Either::Left("err"));
    }

    #[test]
    fn test_applicative_identity_law() {
        let fid = EitherInstance::pure(WrappedFn::from(|x| x));

        let fx = Either::Right(42);
        assert_eq!(fid.clone().apply(fx), Either::Right(42));

        let fx = Either::Left("err");
        assert_eq!(fid.apply(fx), Either::Left("err"));
    }

    #[test]
    fn test_applicative_homomorphism_law() {
        let g = WrappedFn::from(|x| x * 2);
        assert_eq!(
            EitherInstance::<&str>::pure(g.clone()).apply(EitherInstance::pure(3)),
            EitherInstance::pure(g(3))
        );
    }

    #[test]
    fn test_applicative_interchange_law() {
        let g = WrappedFn::from(|x| x + 10);
        let x = 5;

        let lhs = Either::<&str, _>::Right(g.clone()).apply(EitherInstance::pure(x));
        let rhs = EitherInstance::pure(WrappedFn::from(move |g: WrappedFn<i32, i32>| g(x)))
            .apply(Either::Right(g));
        assert_eq!(lhs, rhs);

        let lhs = Either::Left("err");
        let rhs = EitherInstance::pure(WrappedFn::from(move |g: WrappedFn<i32, i32>| g(x)))
            .apply(Either::Left("err"));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_applicative_composition_law() {
        let g = WrappedFn::from(|x| x * 2);
        let h = WrappedFn::from(|x| x + 3);
        let composed = EitherInstance::pure(WrappedFn::curry(compose))
            .apply(Either::Right(g.clone()))
            .apply(Either::Right(h.clone()));

        let xs = Either::Right(4);
        let lhs = composed.clone().apply(xs.clone());
        let rhs = EitherInstance::pure(g).apply(Either::Right(h).apply(xs));
        assert_eq!(lhs, Either::Right(14));
        assert_eq!(lhs, rhs);

        let xs = Either::Left("err");
        let lhs = composed.apply(xs.clone());
        let rhs = EitherInstance::pure(WrappedFn::from(|x| x * 2))
            .apply(Either::Right(WrappedFn::from(|x| x + 3)).apply(xs));
        assert_eq!(lhs, Either::Left("err"));
        assert_eq!(lhs, rhs);
    }
}
