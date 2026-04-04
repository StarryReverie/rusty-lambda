use crate::control::structure::monoid::Monoid;
use crate::control::structure::monoid::last::Last;
use crate::data::maybe::Maybe;

impl<T> Monoid for Last<T> {
    fn empty() -> Self {
        Self(Maybe::Nothing)
    }
}

#[cfg(test)]
mod tests {
    use crate::control::structure::semigroup::Semigroup;

    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(Last::<i32>::empty(), Last(Maybe::Nothing));
    }

    #[test]
    fn test_monoid_left_identity_law() {
        let xs = Last(Maybe::Just(42));
        assert_eq!(Last::<i32>::empty().associate(xs.clone()), xs);

        let xs: Last<i32> = Last(Maybe::Nothing);
        assert_eq!(Last::<i32>::empty().associate(xs.clone()), xs);
    }

    #[test]
    fn test_monoid_right_identity_law() {
        let xs = Last(Maybe::Just(42));
        assert_eq!(xs.clone().associate(Last::<i32>::empty()), xs);

        let xs: Last<i32> = Last(Maybe::Nothing);
        assert_eq!(xs.clone().associate(Last::<i32>::empty()), xs);
    }
}
