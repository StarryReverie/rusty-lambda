use crate::base::numeric::Additive;
use crate::control::structure::monoid::sum::Sum;
use crate::control::structure::semigroup::Semigroup;

impl<T> Semigroup for Sum<T>
where
    T: Additive,
{
    fn associate(self, other: Self) -> Self {
        Sum(self.0 + other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_associate() {
        assert_eq!(Sum(1).associate(Sum(2)), Sum(3));
        assert_eq!(Sum(0).associate(Sum(0)), Sum(0));
        assert_eq!(Sum(-1).associate(Sum(1)), Sum(0));
    }

    #[test]
    fn test_semigroup_associativity_law() {
        let x = Sum(1);
        let y = Sum(2);
        let z = Sum(3);
        assert_eq!(x.associate(y).associate(z), x.associate(y.associate(z)));

        let x = Sum(-1);
        let y = Sum(0);
        let z = Sum(1);
        assert_eq!(x.associate(y).associate(z), x.associate(y.associate(z)));

        let x = Sum(10);
        let y = Sum(20);
        let z = Sum(30);
        assert_eq!(x.associate(y).associate(z), x.associate(y.associate(z)));
    }
}
