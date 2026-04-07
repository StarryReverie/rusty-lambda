use crate::control::structure::semigroup::Semigroup;
use crate::data::maybe::Maybe;

impl<T> Semigroup for Maybe<T>
where
    T: Semigroup,
{
    fn associate(self, other: Self) -> Self {
        match (self, other) {
            (Maybe::Nothing, y) => y,
            (x, Maybe::Nothing) => x,
            (Maybe::Just(x), Maybe::Just(y)) => Maybe::Just(x.associate(y)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::list::List;

    use super::*;

    #[test]
    fn test_associate() {
        let xs: Maybe<List<i32>> = Maybe::Just(List::from(vec![1, 2]));
        let ys: Maybe<List<i32>> = Maybe::Just(List::singleton(3));
        assert_eq!(
            xs.associate(ys.clone()),
            Maybe::Just(List::from(vec![1, 2, 3]))
        );

        let xs: Maybe<List<i32>> = Maybe::Nothing;
        let ys: Maybe<List<i32>> = Maybe::Just(List::singleton(1));
        assert_eq!(xs.associate(ys), Maybe::Just(List::singleton(1)));

        let xs: Maybe<List<i32>> = Maybe::Just(List::singleton(1));
        let ys: Maybe<List<i32>> = Maybe::Nothing;
        assert_eq!(xs.associate(ys), Maybe::Just(List::singleton(1)));

        let xs: Maybe<List<i32>> = Maybe::Nothing;
        let ys: Maybe<List<i32>> = Maybe::Nothing;
        assert_eq!(xs.associate(ys), Maybe::Nothing);
    }

    #[test]
    fn test_semigroup_associativity_law() {
        let xs: Maybe<List<i32>> = Maybe::Just(List::singleton(1));
        let ys: Maybe<List<i32>> = Maybe::Just(List::singleton(2));
        let zs: Maybe<List<i32>> = Maybe::Just(List::singleton(3));
        let lhs = xs.clone().associate(ys.clone()).associate(zs.clone());
        let rhs = xs.associate(ys.associate(zs));
        assert_eq!(lhs, rhs);

        let xs: Maybe<List<i32>> = Maybe::Nothing;
        let ys: Maybe<List<i32>> = Maybe::Just(List::singleton(2));
        let zs: Maybe<List<i32>> = Maybe::Just(List::singleton(3));
        let lhs = xs.clone().associate(ys.clone()).associate(zs.clone());
        let rhs = xs.associate(ys.associate(zs));
        assert_eq!(lhs, rhs);

        let xs: Maybe<List<i32>> = Maybe::Just(List::singleton(1));
        let ys: Maybe<List<i32>> = Maybe::Nothing;
        let zs: Maybe<List<i32>> = Maybe::Just(List::singleton(3));
        let lhs = xs.clone().associate(ys.clone()).associate(zs.clone());
        let rhs = xs.associate(ys.associate(zs));
        assert_eq!(lhs, rhs);

        let xs: Maybe<List<i32>> = Maybe::Just(List::singleton(1));
        let ys: Maybe<List<i32>> = Maybe::Just(List::singleton(2));
        let zs: Maybe<List<i32>> = Maybe::Nothing;
        let lhs = xs.clone().associate(ys.clone()).associate(zs.clone());
        let rhs = xs.associate(ys.associate(zs));
        assert_eq!(lhs, rhs);

        let xs: Maybe<List<i32>> = Maybe::Nothing;
        let ys: Maybe<List<i32>> = Maybe::Nothing;
        let zs: Maybe<List<i32>> = Maybe::Nothing;
        let lhs = xs.clone().associate(ys.clone()).associate(zs.clone());
        let rhs = xs.associate(ys.associate(zs));
        assert_eq!(lhs, rhs);
    }
}
