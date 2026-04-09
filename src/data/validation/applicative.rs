use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::context::applicative::{Applicative, ApplicativeExt};
use crate::control::structure::semigroup::Semigroup;
use crate::data::validation::{Validation, ValidationInstance};

impl<E> Applicative for ValidationInstance<E>
where
    E: Semigroup + Value,
{
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        Validation::Success(x)
    }

    fn apply<A, B, G>(fg: Self::Type<G>, fx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        use Validation::{Failure, Success};
        match (fg, fx) {
            (Success(g), Success(x)) => Success(g.view().call(x)),
            (Failure(e1), Failure(e2)) => Failure(e1.associate(e2)),
            (Failure(e), _) | (_, Failure(e)) => Failure(e),
        }
    }
}

impl<E, A> ApplicativeExt for Validation<E, A>
where
    E: Semigroup + Value,
    A: Value,
{
    type Wrapped = A;
    type Instance = ValidationInstance<E>;
}

#[cfg(test)]
mod tests {
    use crate::base::function::{Curry, WrappedFn, compose};
    use crate::data::list::List;

    use super::*;

    #[test]
    fn test_pure() {
        assert_eq!(
            ValidationInstance::pure(42),
            Validation::<List<i32>, i32>::Success(42)
        );
    }

    #[test]
    fn test_apply() {
        let fg = Validation::<List<i32>, _>::Success(WrappedFn::from(|x| x + 1));
        assert_eq!(fg.apply(Validation::Success(1)), Validation::Success(2));

        let fg: Validation<List<i32>, WrappedFn<i32, i32>> =
            Validation::Failure(List::singleton(1));
        assert_eq!(
            fg.apply(Validation::Success(1)),
            Validation::Failure(List::singleton(1))
        );

        let fg = Validation::Success(WrappedFn::from(|x: i32| x + 1));
        assert_eq!(
            fg.apply(Validation::Failure(List::singleton(2))),
            Validation::Failure(List::singleton(2))
        );

        let fg: Validation<List<i32>, WrappedFn<i32, i32>> =
            Validation::Failure(List::singleton(1));
        assert_eq!(
            fg.apply(Validation::Failure(List::singleton(2))),
            Validation::Failure(List::from(vec![1, 2]))
        );
    }

    #[test]
    fn test_apply_accumulates_errors() {
        let fg: Validation<List<i32>, WrappedFn<i32, i32>> =
            Validation::Failure(List::from(vec![10, 20]));
        let fx = Validation::Failure(List::from(vec![30]));
        assert_eq!(
            fg.apply(fx),
            Validation::Failure(List::from(vec![10, 20, 30]))
        );
    }

    #[test]
    fn test_applicative_identity_law() {
        let fid = ValidationInstance::pure(WrappedFn::from(|x| x));

        let fx = Validation::Success(42);
        assert_eq!(fid.clone().apply(fx), Validation::Success(42));

        let fx = Validation::Failure(List::singleton(1));
        assert_eq!(fid.apply(fx), Validation::Failure(List::singleton(1)));
    }

    #[test]
    fn test_applicative_homomorphism_law() {
        let g = WrappedFn::from(|x| x * 2);
        assert_eq!(
            ValidationInstance::<List<i32>>::pure(g.clone()).apply(ValidationInstance::pure(3)),
            ValidationInstance::pure(g(3))
        );
    }

    #[test]
    fn test_applicative_interchange_law() {
        let g = WrappedFn::from(|x| x + 10);
        let x = 5;

        let lhs: Validation<List<i32>, i32> =
            Validation::Success(g.clone()).apply(ValidationInstance::pure(x));
        let rhs = ValidationInstance::pure(WrappedFn::from(move |g: WrappedFn<i32, i32>| g(x)))
            .apply(Validation::Success(g));
        assert_eq!(lhs, rhs);

        let lhs: Validation<List<i32>, i32> = Validation::Failure(List::singleton(1));
        let rhs = ValidationInstance::pure(WrappedFn::from(move |g: WrappedFn<i32, i32>| g(x)))
            .apply(Validation::Failure(List::singleton(1)));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_applicative_composition_law() {
        let g = WrappedFn::from(|x| x * 2);
        let h = WrappedFn::from(|x| x + 3);
        let composed = ValidationInstance::pure(WrappedFn::curry(compose))
            .apply(Validation::Success(g.clone()))
            .apply(Validation::Success(h.clone()));

        let xs = Validation::Success(4);
        let lhs = composed.clone().apply(xs.clone());
        let rhs = ValidationInstance::pure(g).apply(Validation::Success(h).apply(xs));
        assert_eq!(lhs, Validation::Success(14));
        assert_eq!(lhs, rhs);

        let xs = Validation::Failure(List::singleton(1));
        let lhs = composed.apply(xs.clone());
        let rhs = ValidationInstance::pure(WrappedFn::from(|x| x * 2))
            .apply(Validation::Success(WrappedFn::from(|x| x + 3)).apply(xs));
        assert_eq!(lhs, Validation::Failure(List::singleton(1)));
        assert_eq!(lhs, rhs);
    }
}
