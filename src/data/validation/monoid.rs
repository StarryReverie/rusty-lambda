use crate::base::value::StaticConcurrent;
use crate::control::structure::monoid::Monoid;
use crate::data::validation::Validation;

impl<E, A> Monoid for Validation<E, A>
where
    E: Monoid + StaticConcurrent,
{
    fn empty() -> Self {
        Self::Failure(E::empty())
    }
}

#[cfg(test)]
mod tests {
    use crate::control::structure::semigroup::Semigroup;
    use crate::data::list::List;

    use super::*;

    #[test]
    fn test_empty() {
        let m = Validation::<List<i32>, i32>::empty();
        assert_eq!(m, Validation::Failure(List::empty()));
    }

    #[test]
    fn test_monoid_left_identity_law() {
        let xs = Validation::<List<i32>, i32>::Success(42);
        assert_eq!(Validation::empty().associate(xs.clone()), xs);

        let xs = Validation::<List<i32>, i32>::Failure(List::singleton(1));
        assert_eq!(Validation::empty().associate(xs.clone()), xs);
    }

    #[test]
    fn test_monoid_right_identity_law() {
        let xs = Validation::<List<i32>, i32>::Success(42);
        assert_eq!(xs.clone().associate(Validation::empty()), xs);

        let xs = Validation::<List<i32>, i32>::Failure(List::singleton(1));
        assert_eq!(xs.clone().associate(Validation::empty()), xs);
    }
}
