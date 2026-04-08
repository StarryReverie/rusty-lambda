use crate::control::structure::semigroup::Semigroup;
use crate::data::either::Either;

impl<E, A> Semigroup for Either<E, A> {
    fn associate(self, other: Self) -> Self {
        match (&self, &other) {
            (Either::Right(_), _) => self,
            _ => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::list::List;

    use super::*;

    #[test]
    fn test_associate() {
        let xs: Either<List<i32>, List<i32>> = Either::Right(List::from(vec![1, 2]));
        let ys: Either<List<i32>, List<i32>> = Either::Right(List::singleton(3));
        assert_eq!(xs.associate(ys), Either::Right(List::from(vec![1, 2])));

        let xs: Either<List<i32>, List<i32>> = Either::Right(List::singleton(1));
        let ys: Either<List<i32>, List<i32>> = Either::Left(List::singleton(2));
        assert_eq!(xs.associate(ys), Either::Right(List::singleton(1)));

        let xs: Either<List<i32>, List<i32>> = Either::Left(List::singleton(1));
        let ys: Either<List<i32>, List<i32>> = Either::Right(List::singleton(2));
        assert_eq!(xs.associate(ys), Either::Right(List::singleton(2)));

        let xs: Either<List<i32>, List<i32>> = Either::Left(List::singleton(1));
        let ys: Either<List<i32>, List<i32>> = Either::Left(List::singleton(2));
        assert_eq!(xs.associate(ys), Either::Left(List::singleton(2)));
    }

    #[test]
    fn test_semigroup_associativity_law() {
        let xs: Either<List<i32>, List<i32>> = Either::Right(List::singleton(1));
        let ys: Either<List<i32>, List<i32>> = Either::Right(List::singleton(2));
        let zs: Either<List<i32>, List<i32>> = Either::Right(List::singleton(3));
        let lhs = xs.clone().associate(ys.clone()).associate(zs.clone());
        let rhs = xs.associate(ys.associate(zs));
        assert_eq!(lhs, rhs);

        let xs: Either<List<i32>, List<i32>> = Either::Left(List::singleton(1));
        let ys: Either<List<i32>, List<i32>> = Either::Right(List::singleton(2));
        let zs: Either<List<i32>, List<i32>> = Either::Right(List::singleton(3));
        let lhs = xs.clone().associate(ys.clone()).associate(zs.clone());
        let rhs = xs.associate(ys.associate(zs));
        assert_eq!(lhs, rhs);

        let xs: Either<List<i32>, List<i32>> = Either::Right(List::singleton(1));
        let ys: Either<List<i32>, List<i32>> = Either::Left(List::singleton(2));
        let zs: Either<List<i32>, List<i32>> = Either::Right(List::singleton(3));
        let lhs = xs.clone().associate(ys.clone()).associate(zs.clone());
        let rhs = xs.associate(ys.associate(zs));
        assert_eq!(lhs, rhs);

        let xs: Either<List<i32>, List<i32>> = Either::Right(List::singleton(1));
        let ys: Either<List<i32>, List<i32>> = Either::Right(List::singleton(2));
        let zs: Either<List<i32>, List<i32>> = Either::Left(List::singleton(3));
        let lhs = xs.clone().associate(ys.clone()).associate(zs.clone());
        let rhs = xs.associate(ys.associate(zs));
        assert_eq!(lhs, rhs);

        let xs: Either<List<i32>, List<i32>> = Either::Left(List::singleton(1));
        let ys: Either<List<i32>, List<i32>> = Either::Left(List::singleton(2));
        let zs: Either<List<i32>, List<i32>> = Either::Left(List::singleton(3));
        let lhs = xs.clone().associate(ys.clone()).associate(zs.clone());
        let rhs = xs.associate(ys.associate(zs));
        assert_eq!(lhs, rhs);
    }
}
