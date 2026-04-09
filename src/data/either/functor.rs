use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::structure::functor::{Functor, FunctorExt};
use crate::data::either::{Either, EitherInstance};

impl<E> Functor for EitherInstance<E>
where
    E: Value,
{
    fn fmap<A, B, G>(g: G, fx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        match fx {
            Either::Right(x) => Either::Right(g.view().call(x)),
            Either::Left(e) => Either::Left(e),
        }
    }
}

impl<E, A> FunctorExt for Either<E, A>
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
    use crate::control::structure::functor::fmap;

    use super::*;

    #[test]
    fn test_fmap_right() {
        let fx = Either::<&str, i32>::Right(10);
        assert_eq!(fmap(WrappedFn::from(|x| x + 1), fx), Either::Right(11));
    }

    #[test]
    fn test_fmap_left() {
        let fx = Either::Left("err");
        assert_eq!(
            fmap(WrappedFn::from(|x: i32| x + 1), fx),
            Either::Left("err")
        );
    }

    #[test]
    fn test_functor_identity_law() {
        let id = WrappedFn::from(|x| x);

        let fx = Either::<&str, i32>::Right(42);
        assert_eq!(fmap(id.clone(), fx), Either::Right(42));

        let fx: Either<&str, i32> = Either::Left("err");
        assert_eq!(fmap(id, fx), Either::Left("err"));
    }

    #[test]
    fn test_functor_composition_law() {
        let h = WrappedFn::from(|x| x * 2);
        let g = WrappedFn::from(|x| x + 3);
        let composed = g.clone().compose(h.clone());

        let fx = Either::<&str, i32>::Right(4);
        let lhs = fmap(composed.clone(), fx.clone());
        let rhs = fmap(g.clone(), fmap(h.clone(), fx));
        assert_eq!(lhs, Either::Right(11));
        assert_eq!(lhs, rhs);

        let fx: Either<&str, i32> = Either::Left("err");
        let lhs = fmap(composed, fx.clone());
        let rhs = fmap(g, fmap(h, fx));
        assert_eq!(lhs, Either::Left("err"));
        assert_eq!(lhs, rhs);
    }
}
