use crate::control::monoid::Monoid;
use crate::data::maybe::Maybe;

impl<T> Monoid for Maybe<T>
where
    T: Monoid,
{
    fn empty() -> Self {
        Maybe::Nothing
    }
}

#[cfg(test)]
mod tests {
    use crate::control::semigroup::Semigroup;
    use crate::data::list::List;

    use super::*;

    #[test]
    fn test_monoid_left_identity_law() {
        let xs: Maybe<List<i32>> = Maybe::Just(List::from(vec![1, 2]));
        assert_eq!(Maybe::empty().associate(xs.clone()), xs);

        let xs: Maybe<List<i32>> = Maybe::Nothing;
        assert_eq!(Maybe::empty().associate(xs.clone()), xs);
    }

    #[test]
    fn test_monoid_right_identity_law() {
        let xs: Maybe<List<i32>> = Maybe::Just(List::from(vec![1, 2]));
        assert_eq!(xs.clone().associate(Maybe::empty()), xs);

        let xs: Maybe<List<i32>> = Maybe::Nothing;
        assert_eq!(xs.clone().associate(Maybe::empty()), xs);
    }
}
