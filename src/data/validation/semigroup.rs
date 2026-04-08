use crate::control::structure::semigroup::Semigroup;
use crate::data::validation::Validation;

impl<E, A> Semigroup for Validation<E, A>
where
    E: Semigroup,
{
    fn associate(self, other: Self) -> Self {
        match (self, other) {
            (res @ Self::Success(_), _) | (_, res @ Self::Success(_)) => res,
            (Self::Failure(e1), Self::Failure(e2)) => Self::Failure(e1.associate(e2)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::list::List;

    use super::*;

    #[test]
    fn test_associate() {
        let xs: Validation<List<i32>, i32> = Validation::Success(1);
        let ys: Validation<List<i32>, i32> = Validation::Success(2);
        assert_eq!(xs.associate(ys), Validation::Success(1));

        let xs: Validation<List<i32>, i32> = Validation::Success(1);
        let ys: Validation<List<i32>, i32> = Validation::Failure(List::singleton(2));
        assert_eq!(xs.associate(ys), Validation::Success(1));

        let xs: Validation<List<i32>, i32> = Validation::Failure(List::singleton(1));
        let ys: Validation<List<i32>, i32> = Validation::Success(2);
        assert_eq!(xs.associate(ys), Validation::Success(2));

        let xs: Validation<List<i32>, i32> = Validation::Failure(List::singleton(1));
        let ys: Validation<List<i32>, i32> = Validation::Failure(List::singleton(2));
        assert_eq!(
            xs.associate(ys),
            Validation::Failure(List::from(vec![1, 2]))
        );
    }

    #[test]
    fn test_semigroup_associativity_law() {
        let xs: Validation<List<i32>, i32> = Validation::Success(1);
        let ys: Validation<List<i32>, i32> = Validation::Success(2);
        let zs: Validation<List<i32>, i32> = Validation::Success(3);
        let lhs = xs.clone().associate(ys.clone()).associate(zs.clone());
        let rhs = xs.associate(ys.associate(zs));
        assert_eq!(lhs, rhs);

        let xs: Validation<List<i32>, i32> = Validation::Failure(List::singleton(1));
        let ys: Validation<List<i32>, i32> = Validation::Success(2);
        let zs: Validation<List<i32>, i32> = Validation::Success(3);
        let lhs = xs.clone().associate(ys.clone()).associate(zs.clone());
        let rhs = xs.associate(ys.associate(zs));
        assert_eq!(lhs, rhs);

        let xs: Validation<List<i32>, i32> = Validation::Failure(List::singleton(1));
        let ys: Validation<List<i32>, i32> = Validation::Failure(List::singleton(2));
        let zs: Validation<List<i32>, i32> = Validation::Failure(List::singleton(3));
        let lhs = xs.clone().associate(ys.clone()).associate(zs.clone());
        let rhs = xs.associate(ys.associate(zs));
        assert_eq!(lhs, rhs);

        let xs: Validation<List<i32>, i32> = Validation::Failure(List::singleton(1));
        let ys: Validation<List<i32>, i32> = Validation::Success(2);
        let zs: Validation<List<i32>, i32> = Validation::Failure(List::singleton(3));
        let lhs = xs.clone().associate(ys.clone()).associate(zs.clone());
        let rhs = xs.associate(ys.associate(zs));
        assert_eq!(lhs, rhs);
    }
}
