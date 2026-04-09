use crate::base::value::Value;
use crate::control::context::alternative::{Alternative, AlternativeExt};
use crate::control::structure::monoid::Monoid;
use crate::control::structure::semigroup::Semigroup;
use crate::data::validation::{Validation, ValidationInstance};

impl<E> Alternative for ValidationInstance<E>
where
    E: Monoid + Value,
{
    fn fallback<A>() -> Self::Type<A>
    where
        A: Value,
    {
        Self::Type::<A>::empty()
    }

    fn alt<A>(one: Self::Type<A>, another: Self::Type<A>) -> Self::Type<A>
    where
        A: Value,
    {
        one.associate(another)
    }
}

impl<E, A> AlternativeExt for Validation<E, A>
where
    E: Monoid + Value,
    A: Value,
{
    type Wrapped = A;
    type Instance = ValidationInstance<E>;
}

#[cfg(test)]
mod tests {
    use crate::base::function::WrappedFn;
    use crate::control::structure::functor::fmap;
    use crate::data::list::List;

    use super::*;

    #[test]
    fn test_alternative_left_identity_law() {
        let x: Validation<List<i32>, i32> = Validation::Success(1);
        assert_eq!(x.clone().alt(ValidationInstance::fallback()), x);

        let x: Validation<List<i32>, i32> = ValidationInstance::fallback();
        assert_eq!(x.clone().alt(ValidationInstance::fallback()), x);
    }

    #[test]
    fn test_alternative_right_identity_law() {
        let x: Validation<List<i32>, i32> = Validation::Success(1);
        assert_eq!(ValidationInstance::fallback().alt(x.clone()), x);

        let x: Validation<List<i32>, i32> = ValidationInstance::fallback();
        assert_eq!(ValidationInstance::fallback().alt(x.clone()), x);
    }

    #[test]
    fn test_alternative_associativity_law() {
        let a: Validation<List<i32>, i32> = Validation::Success(1);
        let b: Validation<List<i32>, i32> = Validation::Success(2);
        let c: Validation<List<i32>, i32> = Validation::Success(3);
        let lhs = a.clone().alt(b.clone()).alt(c.clone());
        let rhs = a.alt(b.alt(c));
        assert_eq!(lhs, rhs);

        let a: Validation<List<i32>, i32> = Validation::Failure(List::singleton(1));
        let b: Validation<List<i32>, i32> = Validation::Failure(List::singleton(2));
        let c: Validation<List<i32>, i32> = Validation::Failure(List::singleton(3));
        let lhs = a.clone().alt(b.clone()).alt(c.clone());
        let rhs = a.alt(b.alt(c));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_alternative_left_distributivity_law() {
        let f = WrappedFn::from(|x| x * 2);

        let a: Validation<List<i32>, i32> = Validation::Success(1);
        let b: Validation<List<i32>, i32> = Validation::Success(2);
        let lhs = fmap(f.clone(), a.clone().alt(b.clone()));
        let rhs = fmap(f.clone(), a).alt(fmap(f.clone(), b));
        assert_eq!(lhs, rhs);

        let a: Validation<List<i32>, i32> = Validation::Failure(List::singleton(1));
        let b: Validation<List<i32>, i32> = Validation::Failure(List::singleton(2));
        let lhs = fmap(f.clone(), a.clone().alt(b.clone()));
        let rhs = fmap(f.clone(), a).alt(fmap(f, b));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_alt_accumulates_errors() {
        let a = Validation::Failure(List::from(vec![1, 2]));
        let b = Validation::Failure(List::from(vec![3, 4]));
        assert_eq!(
            a.alt(b),
            Validation::<_, i32>::Failure(List::from(vec![1, 2, 3, 4]))
        );
    }

    #[test]
    fn test_chained_alt() {
        let x: Validation<List<i32>, i32> = ValidationInstance::fallback()
            .alt(Validation::Success(1))
            .alt(Validation::Success(2));
        assert_eq!(x, Validation::Success(1));

        let x: Validation<List<i32>, i32> = Validation::Failure(List::singleton(1))
            .alt(Validation::Failure(List::singleton(2)))
            .alt(Validation::Failure(List::singleton(3)));
        assert_eq!(x, Validation::Failure(List::from(vec![1, 2, 3])));
    }
}
