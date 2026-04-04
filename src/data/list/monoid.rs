use crate::control::monoid::Monoid;
use crate::data::list::List;

impl<T> Monoid for List<T>
where
    T: Clone,
{
    fn empty() -> Self {
        List::empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::control::semigroup::Semigroup;

    use super::*;

    #[test]
    fn test_monoid_left_identity_law() {
        let xs = List::from(vec![1, 2]);
        assert_eq!(List::empty().associate(xs.clone()), xs);

        let xs: List<i32> = List::empty();
        assert_eq!(List::empty().associate(xs.clone()), xs);
    }

    #[test]
    fn test_monoid_right_identity_law() {
        let xs = List::from(vec![1, 2]);
        assert_eq!(xs.clone().associate(List::empty()), xs);

        let xs: List<i32> = List::empty();
        assert_eq!(xs.clone().associate(List::empty()), xs);
    }
}
