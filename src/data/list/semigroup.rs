use crate::control::structure::semigroup::Semigroup;
use crate::data::list::List;

impl<T> Semigroup for List<T>
where
    T: Clone,
{
    fn associate(self, other: Self) -> Self {
        List::append(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append() {
        let xs = List::from(vec![1, 2]);
        let ys = List::singleton(3);
        assert_eq!(xs.associate(ys), List::from(vec![1, 2, 3]));

        let xs: List<i32> = List::empty();
        let ys = List::singleton(1);
        assert_eq!(xs.associate(ys), List::singleton(1));
    }

    #[test]
    fn test_semigroup_associativity_law() {
        let xs = List::singleton(1);
        let ys = List::singleton(2);
        let zs = List::singleton(3);
        let lhs = xs.clone().associate(ys.clone()).associate(zs.clone());
        let rhs = xs.associate(ys.associate(zs));
        assert_eq!(lhs, rhs);

        let xs: List<i32> = List::empty();
        let ys = List::singleton(1);
        let zs = List::singleton(2);
        let lhs = xs.clone().associate(ys.clone()).associate(zs.clone());
        let rhs = xs.associate(ys.associate(zs));
        assert_eq!(lhs, rhs);

        let xs: List<i32> = List::empty();
        let ys: List<i32> = List::empty();
        let zs: List<i32> = List::empty();
        let lhs = xs.clone().associate(ys.clone()).associate(zs.clone());
        let rhs = xs.associate(ys.associate(zs));
        assert_eq!(lhs, rhs);
    }
}
