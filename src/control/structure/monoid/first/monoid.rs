use crate::control::structure::monoid::Monoid;
use crate::control::structure::monoid::first::First;
use crate::data::maybe::Maybe;

impl<T> Monoid for First<T> {
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
        assert_eq!(First::<i32>::empty(), First(Maybe::Nothing));
    }

    #[test]
    fn test_monoid_left_identity_law() {
        let xs = First(Maybe::Just(42));
        assert_eq!(First::<i32>::empty().associate(xs.clone()), xs);

        let xs: First<i32> = First(Maybe::Nothing);
        assert_eq!(First::<i32>::empty().associate(xs.clone()), xs);
    }

    #[test]
    fn test_monoid_right_identity_law() {
        let xs = First(Maybe::Just(42));
        assert_eq!(xs.clone().associate(First::<i32>::empty()), xs);

        let xs: First<i32> = First(Maybe::Nothing);
        assert_eq!(xs.clone().associate(First::<i32>::empty()), xs);
    }
}
