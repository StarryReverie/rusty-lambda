use crate::base::function::ConcurrentFn;
use crate::base::value::{StaticConcurrent, Value};
use crate::control::structure::functor::{Functor, FunctorExt};
use crate::data::validation::{Validation, ValidationInstance};

impl<E> Functor for ValidationInstance<E>
where
    E: StaticConcurrent,
{
    fn fmap<A, B, G>(g: G, fx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        match fx {
            Validation::Success(x) => Validation::Success(g.view().call(x)),
            Validation::Failure(e) => Validation::Failure(e),
        }
    }
}

impl<E, A> FunctorExt for Validation<E, A>
where
    E: StaticConcurrent,
    A: StaticConcurrent,
{
    type Wrapped = A;
    type Instance = ValidationInstance<E>;
}

#[cfg(test)]
mod tests {
    use crate::base::function::WrappedFn;
    use crate::control::structure::functor::fmap;

    use super::*;

    #[test]
    fn test_fmap_success() {
        let fx = Validation::<&str, i32>::Success(10);
        assert_eq!(
            fmap(WrappedFn::from(|x| x + 1), fx),
            Validation::Success(11)
        );
    }

    #[test]
    fn test_fmap_failure() {
        let fx = Validation::Failure("err");
        assert_eq!(
            fmap(WrappedFn::from(|x: i32| x + 1), fx),
            Validation::Failure("err")
        );
    }

    #[test]
    fn test_functor_identity_law() {
        let id = WrappedFn::from(|x| x);

        let fx = Validation::<&str, i32>::Success(42);
        assert_eq!(fmap(id.clone(), fx), Validation::Success(42));

        let fx: Validation<&str, i32> = Validation::Failure("err");
        assert_eq!(fmap(id, fx), Validation::Failure("err"));
    }

    #[test]
    fn test_functor_composition_law() {
        let h = WrappedFn::from(|x| x * 2);
        let g = WrappedFn::from(|x| x + 3);
        let composed = g.clone().compose(h.clone());

        let fx = Validation::<&str, i32>::Success(4);
        let lhs = fmap(composed.clone(), fx.clone());
        let rhs = fmap(g.clone(), fmap(h.clone(), fx));
        assert_eq!(lhs, Validation::Success(11));
        assert_eq!(lhs, rhs);

        let fx: Validation<&str, i32> = Validation::Failure("err");
        let lhs = fmap(composed, fx.clone());
        let rhs = fmap(g, fmap(h, fx));
        assert_eq!(lhs, Validation::Failure("err"));
        assert_eq!(lhs, rhs);
    }
}
