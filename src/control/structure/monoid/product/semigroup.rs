use crate::base::numeric::Multiplicative;
use crate::control::structure::monoid::product::Product;
use crate::control::structure::semigroup::Semigroup;

impl<T> Semigroup for Product<T>
where
    T: Multiplicative,
{
    fn associate(self, other: Self) -> Self {
        Product(self.0 * other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_associate() {
        assert_eq!(Product(2).associate(Product(3)), Product(6));
        assert_eq!(Product(1).associate(Product(1)), Product(1));
        assert_eq!(Product(0).associate(Product(5)), Product(0));
    }

    #[test]
    fn test_semigroup_associativity_law() {
        let x = Product(2);
        let y = Product(3);
        let z = Product(4);
        assert_eq!(x.associate(y).associate(z), x.associate(y.associate(z)));

        let x = Product(-1);
        let y = Product(0);
        let z = Product(1);
        assert_eq!(x.associate(y).associate(z), x.associate(y.associate(z)));

        let x = Product(10);
        let y = Product(20);
        let z = Product(30);
        assert_eq!(x.associate(y).associate(z), x.associate(y.associate(z)));

        let x = Product(1);
        let y = Product(1);
        let z = Product(1);
        assert_eq!(x.associate(y).associate(z), x.associate(y.associate(z)));
    }
}
