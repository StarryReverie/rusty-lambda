use crate::base::numeric::Additive;
use crate::control::structure::monoid::Monoid;
use crate::control::structure::monoid::sum::Sum;

impl<T> Monoid for Sum<T>
where
    T: Additive,
{
    fn empty() -> Self {
        Sum(T::zero())
    }
}

#[cfg(test)]
mod tests {
    use crate::control::structure::semigroup::Semigroup;

    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(Sum::empty(), Sum(0));
    }

    #[test]
    fn test_monoid_left_identity_law() {
        let xs = Sum(5);
        assert_eq!(Sum::empty().associate(xs), xs);
    }

    #[test]
    fn test_monoid_right_identity_law() {
        let xs = Sum(5);
        assert_eq!(xs.associate(Sum::empty()), xs);
    }
}
